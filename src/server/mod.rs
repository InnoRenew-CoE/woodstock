use actix_cors::Cors;
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{
    get, post,
    web::{self, Bytes, Query},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::web::spa;
use futures::{io::WriteAll, FutureExt};
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, env, ffi::OsStr, fs::create_dir_all, path::Path, sync::Mutex, time::Duration};
use tokio::{sync::mpsc, time::sleep};
use tokio_postgres::Client;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use crate::{
    db::{self, build_db_client, retrieve_questions, retrieve_tags, setup_db},
    rag::Rag,
};

struct AppState {
    client: Mutex<Client>,
    rag: Mutex<Rag>,
}

#[derive(MultipartForm)]
struct SubmissionForm {
    #[multipart(limit = "20MB")]
    file: TempFile,
    answers: Text<String>,
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
    let Ok(json) = serde_json::to_string(&structure) else {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok().json(json)
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

    let (tx, rx) = mpsc::channel::<Result<Bytes, Infallible>>(100);
    let mut stream = ReceiverStream::new(rx);

    let Ok(chunks_json) = serde_json::to_string(&result.chunks) else {
        return HttpResponse::InternalServerError().finish();
    };
    println!("JSON: {}", chunks_json);
    let _ = tx.send(Bytes::try_from(chunks_json)).await;

    actix_web::rt::spawn(async move {
        sleep(Duration::from_secs(5)).await;
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

/// Attempts to start the server.
pub async fn start_server(rag: Rag) {
    let server_port = env::var("SERVER_PORT").ok().and_then(|x| x.parse::<u16>().ok()).unwrap_or(6969);
    let client = build_db_client().await;
    setup_db(&client).await;
    create_dir_all(env::var("FILES_FOLDER").unwrap_or("/var/woodstock/files".to_string())).expect("Unable to create the files folder.");

    println!("Server is running on localhost:{}", server_port);
    let state = web::Data::new(AppState {
        client: Mutex::new(client),
        rag: Mutex::new(rag),
    });
    let _ = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(web::scope("/api").service(search).service(submit_answers).service(fetch_questions))
            .service(spa().index_file("public/index.html").static_resources_location("public/").finish())
    })
    .bind(("localhost", server_port))
    .expect("Unable to start the server")
    .run()
    .await;
}
