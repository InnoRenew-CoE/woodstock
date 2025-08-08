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
use crate::shared::file;
use actix_files::Files;
use actix_files::NamedFile;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::AuthResult;
use actix_jwt_auth_middleware::Authority;
use actix_jwt_auth_middleware::FromRequest;
use actix_jwt_auth_middleware::TokenSigner;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_multipart::Multipart;
use actix_web::body::BoxBody;
use actix_web::cookie::time::Time;
use actix_web::cookie::Cookie;
use actix_web::cookie::SameSite;
use actix_web::dev::fn_service;
use actix_web::dev::ResourcePath;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::get;
use actix_web::http::Error;
use actix_web::http::StatusCode;
use actix_web::post;
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web::web::Query;
use actix_web::web::{self};
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::HttpResponseBuilder;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web_lab::web::spa;
use chrono::Duration;
use core::time;
use ed25519_compact::KeyPair;
use futures::TryFutureExt;
use jwt_compact::alg::Ed25519;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::SmtpTransport;
use lettre::Transport;
use passwords::PasswordGenerator;
use serde::Deserialize;
use serde::Serialize;
use sha2::digest::KeyInit;
use sha2::Digest;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::env;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;
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
    #[multipart(limit = "1024MB")]
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

struct FileInformation {
    original_name: String,
    extension: String,
}

/// Stores files in the env["FILES_FOLDER"] folder, submits answers for each file into the database.
#[post("/answers")]
async fn submit_answers(state: web::Data<AppState>, mut payload: Multipart, user: User) -> impl Responder {
    let file_uuid = uuid::Uuid::new_v4().to_string();
    let base_path = std::env::var("FILES_FOLDER").unwrap_or("/var/woodstock/files/".to_string());
    let file_path = format!("{}/{}", base_path, file_uuid);
    let user_id = user.id;
    let mut file_information: Option<FileInformation> = None;
    let mut answers: Vec<Answer> = Vec::new();

    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            if let Some(name) = field.name() {
                println!("Processing {:?}", name);
                match name {
                    "answers" => {
                        let mut data = Vec::new();
                        while let Some(chunk) = field.next().await {
                            if let Ok(chunk) = chunk {
                                data.extend_from_slice(&chunk);
                            }
                        }
                        let Ok(parsed_answers) = serde_json::from_slice::<Vec<Answer>>(&data) else {
                            println!("Unable to parse answers json!");
                            return HttpResponse::BadRequest().finish();
                        };
                        answers.extend(parsed_answers);
                    }
                    "file" => {
                        let Some(content_disposition) = field.content_disposition() else {
                            return HttpResponse::BadRequest().finish();
                        };

                        // Extract filename
                        let original_name = content_disposition
                            .get_filename()
                            .map(|f| f.to_string())
                            .unwrap_or_else(|| "unknown".into());

                        let file_extension = Path::new(&original_name)
                            .extension()
                            .and_then(OsStr::to_str)
                            .unwrap_or("unknown")
                            .to_uppercase();

                        let Ok(mut file) = File::create(&file_path) else {
                            return HttpResponse::BadRequest().finish();
                        };
                        println!("Storing the file into {}", file_path);
                        while let Some(chunk) = field.next().await {
                            if let Ok(data) = chunk {
                                if let Err(error) = file.write(&data) {
                                    println!("Error writing to file: {:?}", error);
                                };
                            }
                        }

                        file_information = Some(FileInformation {
                            original_name,
                            extension: file_extension,
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    let Ok(mut client) = state.client.lock() else {
        return HttpResponse::InternalServerError().finish();
    };

    let Some(FileInformation { original_name, extension }) = file_information else {
        return HttpResponse::BadRequest().finish();
    };

    let Ok(file_id) = db::insert_file(&mut client, &original_name, &file_uuid, &extension, &user_id).await else {
        eprintln!("Unable to insert the file into the database!");
        return HttpResponse::BadRequest().finish();
    };

    for answer in answers {
        if let Err(error) = db::insert_answer(&mut client, answer, &file_id).await {
            println!("Answer insert result: {:?}", error);
        };
    }

    drop(client);
    println!("Processing document_id: {file_id} | {file_uuid} | {original_name}");

    let processable_file_type = match extension.to_ascii_lowercase().as_str() {
        "txt" => RagProcessableFileType::Text,
        "md" => RagProcessableFileType::Markdown,
        "pdf" => RagProcessableFileType::Pdf,
        _ => {
            eprintln!("File must be txt, md or pdf - but is: {}", extension);
            return HttpResponse::BadRequest().finish();
        }
    };

    let rag_file = RagProcessableFile {
        path: PathBuf::from(file_path),
        file_type: processable_file_type,
        internal_id: format!("{file_id}"),
        original_name,
        file_description: None,
        tags: None,
    };

    std::thread::spawn(async move || {
        let _ = match Rag::default().insert(rag_file).await {
            Ok(res) => res,
            Err(e) => {
                println!("rag.insert failed: {:#?}", e.to_string());
            }
        };
    });

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

    if let Err(error) = insert_new_password(client, &login_details.0).await {
        return Ok(HttpResponse::BadRequest().body(error));
    };
    println!("Generated: {},{}", login_details.0.email, password);
    send_mail(&login_details.0.email, &password).await;
    // TODO: Send mail with the password...
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
    send_mail("mihael@regnum.si", "sample").await;
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
            .access_token_lifetime(core::time::Duration::from_secs(86400))
            .refresh_token_lifetime(core::time::Duration::from_secs(86400))
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
            .service(spa().index_file("public/index.html").finish())
        // .service(
        //     Files::new("/", "./")
        //         .prefer_utf8(true)
        //         .index_file("public/index.html")
        //         .default_handler(fn_service(|req: ServiceRequest| async {
        //             let (req, _) = req.into_parts();
        //             let index_path: PathBuf = "public/index.html".into();
        //             let file = NamedFile::open_async(index_path)
        //                 .await?
        //                 .customize()
        //                 .append_header(("Cache-Control", "no-store"));
        //             let res = file.respond_to(&req).map_into_boxed_body();
        //             Ok(ServiceResponse::new(req, res))
        //         })),
        // )
    })
    .bind(("localhost", server_port))
    .expect("Unable to start the server")
    .run()
    .await;
}

async fn send_mail(recipient: &str, password: &str) {
    // "smtp-mail.outlook.com".into(),
    // 587,

    let email = Message::builder()
        .from(Mailbox::new(
            Some("EWCO No Reply".to_string()),
            "ewco-no-reply@innorenew.eu".parse().unwrap(),
        ))
        .to(Mailbox::new(None, recipient.parse().unwrap()))
        .subject("Observatory Access")
        .header(ContentType::TEXT_HTML)
        .body(format!(
            "Your password to access the observatory is <b style='color:#D5451B'>{}</b>. Please visit us at https://observatory.innorenew.eu",
            password
        ))
        .unwrap();

    let creds = Credentials::new("ewco-no-reply@innorenew.eu".to_owned(), env::var("MAIL_SECRET").unwrap());
    let mailer = SmtpTransport::starttls_relay("smtp-mail.outlook.com").unwrap().credentials(creds).build();
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }

    // mailer.send_mail(message).await?;
}
