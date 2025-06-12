use crate::db::build_db_client;
use crate::db::check_login;
use crate::db::insert_new_password;
use crate::db::retrieve_questions;
use crate::db::retrieve_tags;
use crate::db::setup_db;
use crate::db::{self};
use crate::rag::Rag;
use crate::rag::RagProcessableFile;
use crate::rag::RagProcessableFileType;
use actix_cors::Cors;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::AuthResult;
use actix_jwt_auth_middleware::AuthenticationService;
use actix_jwt_auth_middleware::Authority;
use actix_jwt_auth_middleware::FromRequest;
use actix_jwt_auth_middleware::TokenSigner;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Cookie;
use actix_web::cookie::CookieBuilder;
use actix_web::cookie::SameSite;
use actix_web::dev::ResourcePath;
use actix_web::error::ErrorUnauthorized;
use actix_web::get;
use actix_web::guard::Guard;
use actix_web::guard::GuardContext;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::post;
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::web::Query;
use actix_web::web::{self};
use actix_web::App;
use actix_web::Handler;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
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
use std::collections::VecDeque;
use std::convert::Infallible;
use std::env;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_postgres::Client;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

struct AppState {
    client: Mutex<Client>,
    rag: Mutex<Rag>,
    token_signer: TokenSigner<User, Ed25519>,
    invalidated_tokens: Mutex<VecDeque<String>>,
}

#[derive(MultipartForm)]
struct SubmissionForm {
    #[multipart(limit = "20MB")]
    file: TempFile,
    answers: Text<String>,
}
#[derive(Serialize, Deserialize, FromRequest, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub role: UserRole,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub tags: Vec<String>,
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

#[get("/tags")]
async fn fetch_tags(data: Data<AppState>) -> impl Responder {
    let mut client = data.client.lock().unwrap();
    let tags = db::retrieve_tags(&mut client).await.unwrap_or(vec![]);
    HttpResponse::Ok().json(tags)
}

#[get("/files")]
async fn fetch_files(data: Data<AppState>, user: User) -> impl Responder {
    let mut client = data.client.lock().unwrap();
    let files = db::retrieve_files(&user.id, &mut client).await.unwrap_or(vec![]);
    HttpResponse::Ok().json(files)
}

/// Stores files in the env["FILES_FOLDER"] folder, submits answers for each file into the database.
#[post("/answers")]
async fn submit_answers(state: web::Data<AppState>, MultipartForm(form): MultipartForm<SubmissionForm>, user: User) -> impl Responder {
    let tmp_file = form.file;
    let Ok(answers) = serde_json::from_str::<Vec<Answer>>(&form.answers) else {
        eprintln!("Unable to parse answers json!");
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

    let processable_file_type = match file_extension.to_ascii_lowercase().as_str() {
        "txt" => RagProcessableFileType::Text,
        "md" => RagProcessableFileType::Markdown,
        "pdf" => RagProcessableFileType::Pdf,
        _ => {
            eprintln!("File must be txt, md or pdf - but is: {}", file_extension);
            return HttpResponse::BadRequest().finish();
        }
    };

    let file_uuid = uuid::Uuid::new_v4().to_string();
    let base_path = std::env::var("FILES_FOLDER").unwrap_or("/var/woodstock/files/".to_string());
    let file_path = format!("{}/{}", base_path, file_uuid);

    let user_id = user.id;
    let Ok(file_id) = db::insert_file(client, &original_name, &file_uuid, &file_extension, &user_id).await else {
        eprintln!("Unable to insert the file into the database!");
        return HttpResponse::BadRequest().finish();
    };

    // Store the file in the FILES_FOLDER directory using UUID::v4
    if let Err(error) = tmp_file.file.persist(file_path.clone()) {
        println!("{:?}", error);
        return HttpResponse::BadRequest().body(format!("{:?}", error));
    }

    for answer in answers {
        db::insert_answer(client, answer, &file_id).await.unwrap();
    }

    let Ok(rag) = state.rag.lock() else {
        println!("State.rag.lock failed");
        return HttpResponse::InternalServerError().finish();
    };

    println!("Processing document_id: {file_id} | {file_uuid} | {original_name}");
    let rag_file = RagProcessableFile {
        path: PathBuf::from(file_path),
        file_type: processable_file_type,
        internal_id: format!("{file_id}"),
        original_name,
        file_description: None,
        tags: None,
    };

    let _ = match rag.insert(rag_file).await {
        Ok(res) => res,
        Err(e) => {
            println!("rag.insert failed: {:#?}", e.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().finish()
}

#[post("/feedback")]
async fn submit_feedback(state: Data<AppState>, feedback: web::Json<String>, user: User) -> impl Responder {
    let mut client = state.client.lock().unwrap();
    let feedback = feedback.0;
    db::insert_feedback(feedback, &user.id, &mut client).await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[get("/search")]
async fn search(state: web::Data<AppState>, search_query: Query<SearchQuery>) -> impl Responder {
    let Ok(rag) = state.rag.lock() else {
        println!("State.rag.lock failed");
        return HttpResponse::InternalServerError().finish();
    };
    let mut result = match rag.search(search_query.query.clone()).await {
        Ok(res) => res,
        Err(e) => {
            println!("rag.search failed: {:#?}", e.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };

    let (tx, rx) = mpsc::channel::<Result<Bytes, Infallible>>(10_000);
    let stream = ReceiverStream::new(rx);

    let Ok(chunks_json) = serde_json::to_string(&result.chunks) else {
        println!("To json failed");
        return HttpResponse::InternalServerError().finish();
    };

    let _ = tx.send(Bytes::try_from(chunks_json + "\r\n")).await;

    actix_web::rt::spawn(async move {
        sleep(std::time::Duration::from_secs(2)).await;
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
    pub password: Option<String>,
}

#[post("/login")]
async fn login(
    token_signer: web::Data<TokenSigner<User, Ed25519>>,
    data: web::Data<AppState>,
    login_details: web::Json<LoginDetails>,
) -> AuthResult<HttpResponse> {
    let Ok(client) = &mut data.client.lock() else {
        return Ok(HttpResponse::InternalServerError().finish());
    };
    let Ok(user) = check_login(client, login_details.0).await else {
        return Ok(HttpResponse::BadRequest().finish());
    };
    let token_signer = &token_signer;

    let mut access = token_signer.create_access_cookie(&user)?;
    let mut refresh = token_signer.create_refresh_cookie(&user)?;
    access.set_path("/");
    refresh.set_path("/");
    Ok(HttpResponse::Ok().cookie(access).cookie(refresh).finish())
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
    login_details.password = Some(password.clone());

    if let Err(error) = insert_new_password(client, login_details.0).await {
        return Ok(HttpResponse::BadRequest().body(error));
    };
    // TODO: Send mail with the password...
    println!("Sending mail with newly generated password: {:?}", password);
    Ok(HttpResponse::Ok().finish())
}

#[post("/verify")]
async fn verify(data: web::Data<AppState>, user: User, request: HttpRequest) -> HttpResponse {
    let guard = data.invalidated_tokens.lock().expect("Should be able to lock the mutex");
    if let Some(mut access) = request.cookie("access_token") {
        if guard.contains(&access.value().to_string()) {
            access.make_removal();
            access.set_path("/");
            return HttpResponse::Unauthorized().cookie(access).finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[post("/invalidate")]
async fn invalidate(data: web::Data<AppState>, user: User, request: HttpRequest) -> HttpResponse {
    let mut builder = HttpResponseBuilder::new(StatusCode::OK);
    let mut tokens = data.invalidated_tokens.lock().expect("Should be able to lock the mutex.");
    if let Some(mut access) = request.cookie("access_token") {
        if tokens.len() >= 10 {
            tokens.pop_back();
        }
        tokens.push_front(access.value().to_string());
        access.make_removal();
        builder.cookie(access);
    }

    if let Some(mut refresh) = request.cookie("refresh_token") {
        if tokens.len() >= 10 {
            tokens.pop_back();
        }
        tokens.push_front(refresh.value().to_string());
        refresh.make_removal();
        builder.cookie(refresh);
    }
    builder.finish()
}

async fn check_refresh(data: Data<AppState>, request: HttpRequest) -> Result<(), actix_web::Error> {
    println!("Refresh!");
    let guard = data.invalidated_tokens.lock().expect("Should be able to lock the mutex");
    if let Some(mut refresh) = request.cookie("refresh_token") {
        if guard.contains(&refresh.value().to_string()) {
            refresh.make_removal();
            return Err(ErrorUnauthorized("Access Denied"));
        }
    }
    Ok(())
}

/// Attempts to start the server.
pub async fn start_server(rag: Rag) {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
        token_signer: TokenSigner::new()
            .signing_key(secret_key.clone())
            .algorithm(Ed25519)
            .cookie_builder(Cookie::build("", "").path("/").same_site(SameSite::None))
            .build()
            .expect(""),
        invalidated_tokens: Mutex::new(VecDeque::new()),
    });

    let _ = HttpServer::new(move || {
        let authority = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(check_refresh)
            .token_signer(Some(state.token_signer.clone()))
            .verifying_key(public_key)
            .build()
            .expect("");
        // let cors = Cors::default().send_wildcard().allow_any_origin().allow_any_header().allow_any_method();
        App::new()
            // .wrap(cors)
            // .wrap(Logger::default())
            .app_data(state.clone())
            .service(login)
            .service(register)
            .use_jwt(
                authority,
                // .service(
                web::scope("/api")
                    .service(submit_answers)
                    .service(search)
                    .service(fetch_questions)
                    .service(download)
                    .service(verify)
                    .service(fetch_tags)
                    .service(fetch_files)
                    .service(submit_feedback)
                    .service(invalidate),
            )
            .service(spa().index_file("public/index.html").static_resources_location("public/").finish())
    })
    .bind(("localhost", server_port))
    .expect("Unable to start the server")
    .run()
    .await;
}
