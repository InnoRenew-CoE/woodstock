use crate::db::{
    build_db_client, check_login, insert_new_password, retrieve_questions, retrieve_tags, setup_db, {self},
};
use crate::docling;
use crate::rag::agent;
use crate::rag::{Rag, RagProcessableFile, RagProcessableFileType};
use crate::worker::stage_io;
use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
use actix_jwt_auth_middleware::{AuthResult, Authority, FromRequest, TokenSigner};
use actix_multipart::Multipart;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::StatusCode;
use actix_web::web::{
    Bytes, Data, Query, {self},
};
use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use actix_web_lab::web::spa;
use reagent_rs::notifications::NotificationContent;
use serde_json::Value;
use std::collections::HashMap;
use ed25519_compact::KeyPair;
use jwt_compact::alg::Ed25519;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use passwords::PasswordGenerator;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::convert::Infallible;
use std::env;
use std::time::Instant;
use std::ffi::OsStr;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tokio::sync::{mpsc, Mutex};
use tokio_postgres::Client;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;

struct AppState {
    client: Mutex<Client>,
    rag: Mutex<Rag>,
    token_signer: TokenSigner<User, Ed25519>,
    invalidated_tokens: Mutex<VecDeque<String>>,
}

async fn rag_image(path: web::Path<(String, String)>) -> HttpResponse {
    let (document_id, image_name) = path.into_inner();
    if !is_safe_path_segment(&document_id) || !is_safe_path_segment(&image_name) {
        return HttpResponse::BadRequest().finish();
    }

    let image_path = docling::image_root().join(document_id).join(&image_name);
    let Ok(bytes) = tokio::fs::read(&image_path).await else {
        return HttpResponse::NotFound().finish();
    };

    let content_type = mime_guess::from_path(&image_path).first_or_octet_stream();
    HttpResponse::Ok().content_type(content_type.as_ref()).body(bytes)
}

fn is_safe_path_segment(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && !value.contains("..")
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
    let client = &mut state.client.lock().await;
    let questions = retrieve_questions(client).await;
    let available_tags = retrieve_tags(client).await.unwrap_or(HashSet::new());
    let structure = QuestionsStructure {
        available_tags: available_tags.into_iter().collect(),
        questions,
    };
    HttpResponse::Ok().json(&structure)
}

#[get("/tags")]
async fn fetch_tags(data: Data<AppState>) -> impl Responder {
    let mut client = data.client.lock().await;
    let tags = db::retrieve_tags(&mut client).await.unwrap_or(HashSet::new());
    HttpResponse::Ok().json(tags)
}

#[get("/files")]
async fn fetch_files(data: Data<AppState>, user: User) -> impl Responder {
    let mut client = data.client.lock().await;
    let files = db::retrieve_files(&user.id, &mut client).await.unwrap_or(vec![]);
    HttpResponse::Ok().json(files)
}

/// Stores files in the env["FILES_FOLDER"] folder, submits answers for each file into the database.
#[post("/answers")]
async fn submit_answers(state: web::Data<AppState>, mut payload: Multipart, user: User) -> impl Responder {
    let base_path = std::env::var("FILES_FOLDER").unwrap_or("/data/woodstock/files/".to_string());
    let user_id = user.id;
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
                        let file_uuid = uuid::Uuid::new_v4().to_string();
                        let file_path = format!("{}/{}", base_path, file_uuid);
                        println!("User ID: {} | submitted: {}", user_id, file_path);

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

                        println!("Locking for state.client!");
                        let mut client = state.client.lock().await;
                        println!("Lock acquired!");

                        let Ok(file_id) = db::insert_file(&mut client, &original_name, &file_uuid, &file_extension, &user_id).await else {
                            eprintln!("Unable to insert the file into the database!");
                            return HttpResponse::BadRequest().finish();
                        };

                        for answer in &answers {
                            if let Err(error) = db::insert_answer(&mut client, answer, &file_id).await {
                                println!("Answer insert result: {:?}", error);
                            };
                        }

                        println!("Processing document_id: {file_id} | {file_uuid} | {original_name}");

                        let processable_file_type = match RagProcessableFileType::from_extension(&file_extension) {
                            Some(file_type) => file_type,
                            None => {
                                eprintln!(
                                    "Unsupported file extension: {}. Supported extensions: {}",
                                    file_extension,
                                    RagProcessableFileType::supported_extensions().join(", ")
                                );
                                return HttpResponse::BadRequest().finish();
                            }
                        };

                        // Write to staging/new/{uuid} instead of processing inline
                        let staging_dir = std::env::var("STAGING_FOLDER")
                            .unwrap_or_else(|_| "./staging".to_string());
                        let submission_dir = format!("{}/new/{}", staging_dir, file_uuid);
                        if let Err(e) = tokio::fs::create_dir_all(&submission_dir).await {
                            eprintln!("Failed to create staging directory: {e}");
                            return HttpResponse::InternalServerError().finish();
                        }

                        // Copy file to staging dir (keep original at FILES_FOLDER for download)
                        let staging_file_path = format!("{}/file", submission_dir);
                        if let Err(e) = tokio::fs::copy(&file_path, &staging_file_path).await {
                            eprintln!("Failed to copy file to staging: {e}");
                            let _ = tokio::fs::remove_dir_all(&submission_dir).await;
                            return HttpResponse::InternalServerError().finish();
                        }

                        // Write metadata.json
                        let rag_file = RagProcessableFile {
                            path: PathBuf::from(&staging_file_path),
                            file_type: processable_file_type,
                            internal_id: format!("{file_id}"),
                            original_name,
                            file_description: None,
                            tags: None,
                        };
                        let metadata_path = format!("{}/metadata.json", submission_dir);
                        if let Err(e) = stage_io::write_json(
                            &PathBuf::from(&metadata_path),
                            &rag_file,
                        ).await {
                            eprintln!("Failed to write metadata.json: {e}");
                            let _ = tokio::fs::remove_dir_all(&submission_dir).await;
                            return HttpResponse::InternalServerError().finish();
                        }

                        // Write answers.json
                        let answers_path = format!("{}/answers.json", submission_dir);
                        if let Err(e) = stage_io::write_json(
                            &PathBuf::from(&answers_path),
                            &answers,
                        ).await {
                            eprintln!("Failed to write answers.json: {e}");
                        }

                        println!("Staged submission {file_uuid} in {submission_dir}");
                        drop(client);

                    }
                    _ => (),
                }
            }
        }
    }

    HttpResponse::Ok().finish()
}

#[post("/submit")]
async fn submit_csv(state: web::Data<AppState>, user: User, mut payload: Multipart) -> impl Responder {
    let base_path = std::env::var("FILES_FOLDER").unwrap_or("/data/woodstock/files/".to_string());
    let user_id = user.id;

    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            if let Some(name) = field.name() {
                println!("Processing {:?}", name);
                match name {
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

                        let file_uuid = uuid::Uuid::new_v4().to_string();
                        let file_path = format!("{}/{}", base_path, file_uuid);

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
                        let mut client = state.client.lock().await;

                        let Ok(_) = db::insert_template(file_uuid, &user_id, &mut client).await else {
                            eprintln!("Unable to insert the file into the database!");
                            return HttpResponse::BadRequest().finish();
                        };
                    }
                    _ => (),
                }
            }
        }
    }

    HttpResponse::Ok().finish()
}

#[post("/feedback")]
async fn submit_feedback(state: Data<AppState>, feedback: web::Json<String>, user: User) -> impl Responder {
    let mut client = state.client.lock().await;
    let feedback = feedback.0;
    db::insert_feedback(feedback, &user.id, &mut client).await;
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
}

#[derive(Deserialize)]
struct ChatRequest {
    query: String,
    history: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}



fn spawn_agent_search(
    ndjson_tx: mpsc::Sender<Result<Bytes, Infallible>>,
    query: String,
    history: Vec<reagent_rs::Message>,
) {
    actix_web::rt::spawn(async move {
        let start = Instant::now();
        let (chunks_tx, mut chunks_rx) = tokio::sync::mpsc::channel::<Value>(16);

        println!("[SEARCH] Building agent...");
        let (mut agent, mut notification_rx) = match agent::build_search_agent(chunks_tx).await {
            Ok((a, n)) => {
                println!("[SEARCH] Agent built successfully");
                (a, n)
            }
            Err(e) => {
                eprintln!("[SEARCH] ERROR — Failed to build agent: {e}");
                let err = serde_json::json!({"type": "error", "value": e.to_string(), "display": false});
                let _ = ndjson_tx.send(Ok(Bytes::from(serde_json::to_string(&err).unwrap() + "\n"))).await;
                return;
            }
        };

        // push history into agent so it sees prior conversation
        let history_count = history.len();
        for msg in history {
            agent.history.push(msg);
        }
        if history_count > 0 {
            println!("[SEARCH] Injected {history_count} history messages into agent");
        }

        // spawn task to forward chunks from the tool to the ndjson stream
        let ndjson_tx_clone = ndjson_tx.clone();
        tokio::spawn(async move {
            while let Some(msg) = chunks_rx.recv().await {
                println!("[SEARCH] Forwarding {} chunks message to stream", msg.get("value").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0));
                let line = serde_json::to_string(&msg).unwrap_or_default() + "\n";
                if ndjson_tx_clone.send(Ok(Bytes::from(line))).await.is_err() {
                    break;
                }
            }
        });

        // spawn task to forward agent notifications to the ndjson stream
        let ndjson_tx_clone = ndjson_tx.clone();
        tokio::spawn(async move {
            let mut token_count = 0;
            while let Some(notification) = notification_rx.recv().await {
                match notification.content {
                    NotificationContent::Token(t) => {
                        token_count += 1;
                        let msg = serde_json::json!({
                            "type": "token",
                            "value": t.value,
                            "display": true,
                        });
                        let line = serde_json::to_string(&msg).unwrap_or_default() + "\n";
                        if ndjson_tx_clone.send(Ok(Bytes::from(line))).await.is_err() {
                            return;
                        }
                    }
                    NotificationContent::ToolCallRequest(tool) => {
                        println!("[SEARCH] Agent called tool: {}", tool.function.name);
                    }
                    NotificationContent::ToolCallSuccessResult(result) => {
                        println!("[SEARCH] Tool call succeeded — result length: {}", result.len());
                    }
                    NotificationContent::ToolCallErrorResult(error) => {
                        eprintln!("[SEARCH] Tool call error: {error}");
                    }
                    NotificationContent::Done(success, response) => {
                        println!("[SEARCH] Agent done — success: {success}, token_count: {token_count}, response_len: {:?}, elapsed: {:?}", response.as_ref().map(|r| r.len()), start.elapsed());
                        let msg = serde_json::json!({"type": "done", "display": false});
                        let line = serde_json::to_string(&msg).unwrap_or_default() + "\n";
                        let _ = ndjson_tx_clone.send(Ok(Bytes::from(line))).await;
                        break;
                    }
                    _ => {}
                }
            }
        });

        println!("[SEARCH] Invoking agent flow...");
        let prompt_data = HashMap::from([("query", query.as_str())]);
        if let Err(e) = agent.invoke_flow_with_template(prompt_data).await {
            eprintln!("[SEARCH] ERROR — Agent invocation failed: {e}");
            let err = serde_json::json!({"type": "error", "value": e.to_string(), "display": false});
            let line = serde_json::to_string(&err).unwrap_or_default() + "\n";
            let _ = ndjson_tx.send(Ok(Bytes::from(line))).await;
        }
        println!("[SEARCH] Agent invocation completed — total time: {:?}", start.elapsed());
    });
}

#[get("/search")]
async fn search_get(state: web::Data<AppState>, search_query: Query<SearchQuery> /* , user: User */) -> impl Responder {
    let query = &search_query.0.query;
    println!("[SEARCH] GET query: \"{query}\"");

    let mut client = state.client.lock().await;
    if let Err(error) = db::insert_query(1, &client, query).await {
        eprintln!("[SEARCH] Inserting query log failed: {:?}", error);
        return HttpResponse::InternalServerError().finish();
    }
    drop(client);

    let (ndjson_tx, ndjson_rx) = mpsc::channel::<Result<Bytes, Infallible>>(10_000);
    let stream = ReceiverStream::new(ndjson_rx);

    spawn_agent_search(ndjson_tx, query.clone(), vec![]);

    HttpResponse::Ok().content_type("application/x-ndjson").streaming(stream)
}

#[post("/search")]
async fn search_post(state: web::Data<AppState>, body: web::Json<ChatRequest>) -> impl Responder {
    println!("[SEARCH] POST query: \"{}\" with {} history messages", body.query, body.history.len());

    let mut client = state.client.lock().await;
    if let Err(error) = db::insert_query(1, &client, &body.query).await {
        eprintln!("[SEARCH] Inserting query log failed: {:?}", error);
        return HttpResponse::InternalServerError().finish();
    }
    drop(client);

    let history: Vec<reagent_rs::Message> = body
        .history
        .iter()
        .filter_map(|m| match m.role.as_str() {
            "user" => Some(reagent_rs::Message::user(&m.content)),
            "assistant" => Some(reagent_rs::Message::assistant(&m.content)),
            _ => {
                eprintln!("[SEARCH] Unknown history role: {}", m.role);
                None
            }
        })
        .collect();

    let (ndjson_tx, ndjson_rx) = mpsc::channel::<Result<Bytes, Infallible>>(10_000);
    let stream = ReceiverStream::new(ndjson_rx);

    spawn_agent_search(ndjson_tx, body.query.clone(), history);

    HttpResponse::Ok().content_type("application/x-ndjson").streaming(stream)
}

#[get("/download/{file_id}")]
pub async fn download(state: Data<AppState>, file_id: web::Path<String>) -> HttpResponse {
    let mut client = state.client.lock().await;

    let file = db::find_file(file_id.parse().unwrap(), &mut client).await;
    println!("File found to be downloaded: {:?}", file);
    if let Ok(file_info) = file {
        let path = format!("/data/woodstock/files/{}", file_info.name).replace(".", "");
        if !path.starts_with("/data/woodstock/files/") {
            return HttpResponse::InternalServerError().finish();
        }

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Opening file error: {:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_err() {
            return HttpResponse::InternalServerError().finish();
        }

        let filename = Path::new(&file_info.original_name)
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("download");

        return HttpResponse::Ok()
            .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
            .body(buffer);
    }

    HttpResponse::NotFound().finish()
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
    let mut client = data.client.lock().await;
    let Ok(user) = check_login(&mut client, login_details.0).await else {
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
    let mut client = data.client.lock().await;

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

    if let Err(error) = insert_new_password(&mut client, &login_details.0).await {
        return Ok(HttpResponse::BadRequest().body(error));
    };
    println!("Generated: {},{}", login_details.0.email, password);
    send_mail(&login_details.0.email, &password).await;
    // TODO: Send mail with the password...
    Ok(HttpResponse::Ok().finish())
}

#[post("/verify")]
async fn verify(data: web::Data<AppState>, _: User, request: HttpRequest) -> HttpResponse {
    let guard = data.invalidated_tokens.lock().await;
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
async fn invalidate(data: web::Data<AppState>, _: User, request: HttpRequest) -> HttpResponse {
    let mut builder = HttpResponseBuilder::new(StatusCode::OK);
    let mut tokens = data.invalidated_tokens.lock().await;
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

#[derive(Serialize, Deserialize, Debug)]
struct PostBody {
    id: Option<i32>,
    title: String,
    body: String,
}

#[post("/collaborate")]
async fn new_post(state: web::Data<AppState>, user: User, body: web::Json<PostBody>) -> HttpResponse {
    let mut client = state.client.lock().await;
    let post = body.0;
    let result = db::upsert_post(post.id, post.title, post.body, &user.id, &mut client).await;
    let mut builder = HttpResponseBuilder::new(if result.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    });
    builder.finish()
}

#[get("/posts")]
async fn retrieve_posts(state: web::Data<AppState>) -> HttpResponse {
    let mut client = state.client.lock().await;

    let posts = db::get_posts(&mut client).await;
    let mut builder = HttpResponseBuilder::new(StatusCode::OK);

    if let Ok(posts) = posts {
        return builder.json(posts);
    }
    builder.finish()
}

async fn check_refresh(data: Data<AppState>, request: HttpRequest) -> Result<(), actix_web::Error> {
    println!("Refresh!");
    let guard = data.invalidated_tokens.lock().await;
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
    // send_mail("mihael@regnum.si", "sample").await;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_port = env::var("SERVER_PORT").ok().and_then(|x| x.parse::<u16>().ok()).unwrap_or(6969);
    let client = build_db_client().await;
    setup_db(&client).await;
    create_dir_all(env::var("FILES_FOLDER").unwrap_or("/var/woodstock/files".to_string())).expect("Unable to create the files folder.");
    create_dir_all(docling::image_root()).expect("Unable to create the RAG image folder.");

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
            .service(retrieve_posts)
            .route("/rag/images/{document_id}/{image_name}", web::get().to(rag_image))
            .route("/audio", web::get().to(audio_ws))
            .route("/transcribe", web::post().to(transcribe_audio))
            .service(web::scope("/chat").service(search_get).service(search_post).service(download))
            .use_jwt(
                authority,
                // .service(
                web::scope("/api")
                    .service(submit_answers)
                    .service(submit_csv)
                    .service(fetch_questions)
                    .service(verify)
                    .service(fetch_tags)
                    .service(fetch_files)
                    .service(submit_feedback)
                    .service(invalidate)
                    .service(new_post),
            )
            .service(spa().index_file("public/index.html").static_resources_location("public/").finish())
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
        Err(e) => eprintln!("Could not send email: {e:?}"),
    }

    // mailer.send_mail(message).await?;
}

use awc::ws;
use futures::SinkExt;

pub async fn audio_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    println!("WebSocket connection attempt from {:?}", req.peer_addr());

    let (res, mut client_session, mut client_stream) = actix_ws::handle(&req, stream)?;

    println!("WebSocket connection");

    actix_web::rt::spawn(async move {
        let whisper_conn = awc::Client::new().ws("ws://localhost:9090").connect().await;

        let (_, mut whisper) = match whisper_conn {
            Ok(c) => c,
            Err(e) => {
                println!("Failed to connect to WhisperLive: {e}");
                return;
            }
        };

        println!("Connected to WhisperLive");

        loop {
            tokio::select! {
                Some(Ok(msg)) = client_stream.recv() => {
                    println!("Client -> Whisper: {:?}", msg);
                    match msg {
                        actix_ws::Message::Binary(bytes) => {
                            whisper.send(ws::Message::Binary(bytes)).await.ok();
                        }
                        actix_ws::Message::Text(text) => {
                            println!("Forwarding text: {text}");
                            whisper.send(ws::Message::Text(text)).await.ok();
                        }
                        actix_ws::Message::Close(_) => {
                            whisper.send(ws::Message::Close(None)).await.ok();
                            client_session.close(None).await.ok();
                            break;
                        }
                        actix_ws::Message::Ping(b) => { client_session.pong(&b).await.ok(); }
                        _ => {}
                    }
                }
                Some(Ok(msg)) = whisper.next() => {
                    println!("Whisper -> Client: {:?}", msg);
                    match msg {
                        ws::Frame::Text(text) => {
                            let s = String::from_utf8_lossy(&text).to_string();
                            client_session.text(s).await.ok();
                        }
                        ws::Frame::Binary(bytes) => {
                            client_session.binary(bytes).await.ok();
                        }
                        ws::Frame::Close(c) => {
                            println!("Whisper closed: {:?}", c);
                            client_session.close(None).await.ok();
                            break;
                        }
                        ws::Frame::Ping(b) => { whisper.send(ws::Message::Pong(b)).await.ok(); }
                        _ => {}
                    }
                }
                else => {
                    println!("Both streams ended, breaking");
                    break;
                }
            }
        }
        println!("Loop ended");
    });

    Ok(res)
}

use reqwest::multipart;

pub async fn transcribe_audio(mut payload: Multipart) -> HttpResponse {
    let mut audio_bytes: Vec<u8> = vec![];
    let mut filename = "recording.wav".to_string();

    while let Some(Ok(mut field)) = payload.next().await {
        if field.name() == Some("file") {
            filename = "recording.wav".to_string();
            while let Some(Ok(chunk)) = field.next().await {
                audio_bytes.extend_from_slice(&chunk);
            }
        }
    }

    let form = multipart::Form::new()
        .part(
            "file",
            multipart::Part::bytes(audio_bytes).file_name(filename).mime_str("audio/wav").unwrap(),
        )
        .text("model", "whisper-1")
        .text("language", "en");

    let res = reqwest::Client::new()
        .post("http://localhost:8000/v1/audio/transcriptions")
        .multipart(form)
        .send()
        .await;

    match res {
        Ok(r) => {
            let body = r.json::<serde_json::Value>().await.unwrap_or_default();
            HttpResponse::Ok().json(body)
        }
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() })),
    }
}
