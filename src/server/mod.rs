use crate::db::build_db_client;
use crate::db::check_login;
use crate::db::insert_new_password;
use crate::db::retrieve_questions;
use crate::db::retrieve_tags;
use crate::db::setup_db;
use crate::db::{self};
use crate::rag::Rag;
use actix_cors::Cors;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::AuthResult;
use actix_jwt_auth_middleware::Authority;
use actix_jwt_auth_middleware::TokenSigner;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::cookie::Cookie;
use actix_web::dev::ResourcePath;
use actix_web::get;
use actix_web::guard::Guard;
use actix_web::guard::GuardContext;
use actix_web::post;
use actix_web::web::Bytes;
use actix_web::web::Query;
use actix_web::web::{self};
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web_lab::web::spa;
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use passwords::PasswordGenerator;
use serde::Deserialize;
use serde::Serialize;
use sha2::digest::KeyInit;
use sha2::Digest;
use sha2::Sha256;
use std::convert::Infallible;
use std::env;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_postgres::Client;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

struct AppState {
    client: Mutex<Client>,
    rag: Mutex<Rag>,
    token_signer: TokenSigner<User, Ed25519>,
}

#[derive(MultipartForm)]
struct SubmissionForm {
    #[multipart(limit = "20MB")]
    file: TempFile,
    answers: Text<String>,
}
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize)]
pub enum UserRole {
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub question_type: i32,
    pub possible_answers: Vec<SelectionAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionAnswer {
    pub id: i32,
    pub question_id: i32,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub question_id: i32,
    pub text: Option<String>,
    pub selection: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAnswer {
    file: String,
    answers: Vec<Answer>,
}

#[derive(Serialize)]
struct QuestionsStructure {
    questions: Vec<Question>,
    available_tags: Vec<String>,
}

/// Returns a JSON representation of questions stored in the database.
#[get("/questions")]
async fn fetch_questions(state: web::Data<AppState>) -> impl Responder {
    let Ok(client) = &mut state.client.lock() else {
        return HttpResponse::InternalServerError().finish();
    };
    let questions = retrieve_questions(client).await;
    let available_tags = retrieve_tags(client).await.unwrap_or(vec![]);
    let structure = QuestionsStructure { available_tags, questions };
    HttpResponse::Ok().json(&structure)
}

/// Stores files in the env["FILES_FOLDER"] folder, submits answers for each file into the database.
#[post("/answers")]
async fn submit_answers(state: web::Data<AppState>, MultipartForm(form): MultipartForm<SubmissionForm>) -> impl Responder {
    let tmp_file = form.file;
    let Ok(answers) = serde_json::from_str::<Vec<Answer>>(&form.answers) else {
        return HttpResponse::BadRequest().finish();
    };
    let Ok(client) = &mut state.client.lock() else {
        return HttpResponse::InternalServerError().finish();
    };

    let original_name = tmp_file.file_name.unwrap_or("unknown".to_string());
    let file_extension = Path::new(&original_name)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("unknown")
        .to_uppercase();
    let file_uuid = uuid::Uuid::new_v4().to_string();
    let base_path = std::env::var("FILES_FOLDER").unwrap_or("/var/woodstock/files/".to_string());
    let file_path = format!("{}/{}", base_path, file_uuid);
    let user_id = 1i32;
    let Ok(file_id) = db::insert_file(client, &original_name, &file_uuid, &file_extension, &user_id).await else {
        return HttpResponse::BadRequest().finish();
    };

    // Store the file in the FILES_FOLDER directory using UUID::v4
    if let Err(error) = tmp_file.file.persist(file_path) {
        println!("{:?}", error);
        return HttpResponse::BadRequest().body(format!("{:?}", error));
    }

    for answer in answers {
        db::insert_answer(client, answer, &file_id).await;
    }

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[get("/search")]
async fn search(state: web::Data<AppState>, search_query: Query<SearchQuery>) -> impl Responder {
    let Ok(rag) = state.rag.lock() else {
        return HttpResponse::InternalServerError().finish();
    };
    let Ok(mut result) = rag.search(search_query.query.clone()).await else {
        return HttpResponse::InternalServerError().finish();
    };

    let (tx, rx) = mpsc::channel::<Result<Bytes, Infallible>>(10_000);
    let stream = ReceiverStream::new(rx);

    let Ok(chunks_json) = serde_json::to_string(&result.chunks) else {
        return HttpResponse::InternalServerError().finish();
    };

    let _ = tx.send(Bytes::try_from(chunks_json + "\r\n")).await;

    actix_web::rt::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        while let Some(res) = result.stream.next().await {
            if let Ok(responses) = res {
                for resp in responses {
                    let data = Bytes::copy_from_slice(resp.response.as_bytes());
                    let _ = tx.send(Ok(data)).await;
                }
            }
        }
    });

    HttpResponse::Ok().content_type("text/plain").streaming(stream)
}

#[get("/download/{file_id}")]
pub async fn download(file_id: web::Path<String>) -> HttpResponse {
    let path = format!("/var/woodstock/files/{}", file_id.path()).replace(".", "");

    if !path.starts_with("/var/woodstock/files/") {
        return HttpResponse::InternalServerError().finish();
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return HttpResponse::InternalServerError().finish(),
    };

    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let filename = path.split("/").last().unwrap_or("download");
    let filename = format!("{filename}.pdf");

    HttpResponse::Ok()
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
        .body(buffer)
}

#[derive(Deserialize, Debug)]
pub struct LoginDetails {
    pub email: String,
    pub password: String,
}

#[post("/login")]
async fn login(data: web::Data<AppState>, login_details: web::Json<LoginDetails>) -> AuthResult<HttpResponse> {
    let Ok(client) = &mut data.client.lock() else {
        return Ok(HttpResponse::InternalServerError().finish());
    };
    println!("Login has been called!");
    let Ok(user) = check_login(client, login_details.0).await else {
        return Ok(HttpResponse::BadRequest().finish());
    };

    let token_signer = &data.token_signer;

    Ok(HttpResponse::Ok()
        .cookie(token_signer.create_access_cookie(&user)?)
        .cookie(token_signer.create_refresh_cookie(&user)?)
        .finish())
}

#[post("/register")]
async fn register(data: web::Data<AppState>, mut login_details: web::Json<LoginDetails>) -> AuthResult<HttpResponse> {
    let Ok(client) = &mut data.client.lock() else {
        return Ok(HttpResponse::InternalServerError().finish());
    };
    let pg = PasswordGenerator {
        length: 12,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    let password = pg.generate_one().expect("Unable to generate a secure password");
    login_details.password = password.clone();

    if let Err(error) = insert_new_password(client, login_details.0).await {
        return Ok(HttpResponse::BadRequest().body(error));
    };
    // TODO: Send mail with the password...
    println!("Sending mail with newly generated password: {:?}", password);
    Ok(HttpResponse::Ok().finish())
}

/// Attempts to start the server.
pub async fn start_server(rag: Rag) {
    let server_port = env::var("SERVER_PORT").ok().and_then(|x| x.parse::<u16>().ok()).unwrap_or(6969);
    let client = build_db_client().await;
    setup_db(&client).await;
    create_dir_all(env::var("FILES_FOLDER").unwrap_or("/var/woodstock/files".to_string())).expect("Unable to create the files folder.");

    println!("Server is running on localhost:{}", server_port);
    let KeyPair {
        pk: public_key,
        sk: secret_key,
    } = KeyPair::generate();
    let state = web::Data::new(AppState {
        client: Mutex::new(client),
        rag: Mutex::new(rag),
        token_signer: TokenSigner::new().signing_key(secret_key.clone()).algorithm(Ed25519).build().expect(""),
    });

    let _ = HttpServer::new(move || {
        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(state.token_signer.clone()))
            .verifying_key(public_key)
            .build()
            .expect("");
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(login)
            .service(register)
            .use_jwt(
                authority,
                web::scope("/api")
                    .service(search)
                    .service(submit_answers)
                    .service(fetch_questions)
                    .service(download),
            )
            .service(spa().index_file("public/index.html").static_resources_location("public/").finish())
    })
    .bind(("localhost", server_port))
    .expect("Unable to start the server")
    .run()
    .await;
}
