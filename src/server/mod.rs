use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::web::spa;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls};

struct AppState {
    client: Mutex<Client>,
}

#[get("/questions")]
async fn fetch_questions(state: web::Data<AppState>) -> impl Responder {
    let Ok(client) = &mut state.client.lock() else {
        return HttpResponse::InternalServerError().finish();
    };
    let questions = retrieve_questions(client).await;
    let Ok(json) = serde_json::to_string(&questions) else {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok().json(json)
}

pub async fn start_server() {
    let state = web::Data::new(AppState {
        client: Mutex::new(build_db_client().await),
    });
    println!("DB Client setup.");
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api").service(fetch_questions))
            .service(
                spa()
                    .index_file("public/index.html")
                    .static_resources_location("public/")
                    .finish(),
            )
    })
    .bind(("localhost", 8080))
    .expect("Unable to start the server")
    .run()
    .await;
}

pub async fn build_db_client() -> Client {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
            .await
            .expect("Unable to conenct to database");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}

pub async fn retrieve_questions(client: &mut Client) -> Vec<Question> {
    let mut questions: HashMap<i32, Question> = HashMap::new();
    let questions_query_results = client
        .query("select * from questions", &[])
        .await
        .expect("Unable to query questions.")
        .into_iter()
        .map(|row| Question {
            id: row.get("id"),
            title: row.get("title"),
            text: row.get("text"),
            question_type: row.get("type"),
            possible_answers: vec![],
        })
        .collect::<Vec<Question>>();
    for question in questions_query_results {
        questions.insert(question.id, question);
    }

    for row in client
        .query("select * from selection_answers", &[])
        .await
        .expect("Unable to query selection answers")
        .into_iter()
    {
        let answer = SelectionAnswer {
            id: row.get("id"),
            question_id: row.get("question_id"),
            value: row.get("value"),
        };
        let existing_question = questions.get_mut(&answer.question_id);
        if let Some(question) = existing_question {
            question.possible_answers.push(answer);
        }
    }
    questions.into_iter().map(|(_id, q)| q).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    id: i32,
    title: String,
    text: String,
    question_type: i32,
    possible_answers: Vec<SelectionAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionAnswer {
    id: i32,
    question_id: i32,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    id: i32,
    question_id: i32,
    text: Option<String>,
    selection: Vec<i32>,
}
