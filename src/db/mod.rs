use crate::server::Answer;
use crate::server::LoginDetails;
use crate::server::Question;
use crate::server::SelectionAnswer;
use crate::server::User;
use crate::server::UserRole;
use anyhow::bail;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Utc;
use std::collections::HashMap;
use std::env;
use std::io::Error;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tokio_postgres::types::Date;
use tokio_postgres::types::Timestamp;
use tokio_postgres::Client;
use tokio_postgres::NoTls;

const INSERT_FILES_QUERY: &'static str = r#"insert into files (original_name, name, type, submitted_by) values ($1, $2, $3, $4) returning id"#;
const INSERT_SELECTION_ANSWER_QUERY: &'static str = r#"insert into answers_selection (file_id, question_id, answer_id) values ($1, $2, $3)"#;
const INSERT_TEXT_ANSWER_QUERY: &'static str = r#"insert into answers_text (file_id, question_id, text) values ($1, $2, $3)"#;
const SELECT_USER: &'static str = "select users.id as id, user_roles.title as role from users left join user_roles on users.role = user_roles.id where email = $1 and password = $2";
const SET_PASSWORD: &'static str = "update users set password = $2, last_password_change = now() where email = $1;";
const SELECT_QUESTIONS: &'static str = "select * from questions";
const SELECT_QUESTION_OPTIONS: &'static str = "select * from question_options";
const SELECT_DISTINCT_TAGS: &'static str = "select distinct (value) from tags";

const SELECT_TOTAL_ANSWERS: &'static str =
    "select SUM(coalesce((select count(*) from answers_text)) + (select count(*) from answers_selection)) as total";

const SELECT_FILE_TYPE_STATISTICS: &'static str = "select type,
       round(100 * count(*) /
             Sum(count(*)) OVER (),
             1) AS percentage
from files
group by type
order by percentage DESC
limit 5
";

const TABLES_SETUP: &'static str = r#"

create table if not exists user_roles
(
    id    serial primary key,
    title varchar(50)
);

create table if not exists users
(
    id                   serial primary key,
    email                varchar(100),
    password             varchar(150),
    joined_on            date default now(),
    role                 int references user_roles,
    last_password_change date
);

create table if not exists questions
(
    id    serial primary key,
    title text unique,
    text  text,
    type  int
);

create table if not exists question_options
(
    id          serial primary key,
    question_id int references questions,
    value       varchar(100)
);

create table if not exists files
(
    id              serial primary key,
    internal_id     varchar(200) default null,
    original_name   text,
    name            varchar(200),
    submission_date date default now(),
    type            varchar(30),
    submitted_by    int references users
);

create table if not exists answers_selection
(
    id          serial primary key,
    file_id     int references files,
    question_id int references questions,
    answer_id   int references question_options
);

create table if not exists answers_text
(
    id          serial primary key,
    file_id     int references files,
    question_id int references questions,
    text        text
);
"#;

/// Attempts to create all tables required by this software.
pub async fn setup_db(client: &Client) {
    println!("Executing tables setup.");
    if let Err(error) = client.batch_execute(TABLES_SETUP).await {
        panic!("Unable to setup the database tables. {:?}", error);
    }
    println!("Executed tables setup.");
}

/// Attempts to connect to the database and return the built Client.
pub async fn build_db_client() -> Client {
    let db_host = env::var("DB_HOST").expect("Missing DB_HOST in .env!");
    let db_user = env::var("DB_USER").expect("Missing DB_USER in .env!");
    let db_password = env::var("DB_PASSWORD").expect("Missing DB_PASSWORD in .env!");
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| {
        println!("Defaulting to port 5432!");
        return "5432".to_string();
    });

    let connection_string = format!("host={db_host} user={db_user} password={db_password} port={db_port}");
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .expect("Unable to conenct to database");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}

/// Retrieves a vector of [Question] from the database.
pub async fn retrieve_questions(client: &mut Client) -> Vec<Question> {
    let mut questions: HashMap<i32, Question> = HashMap::new();
    let questions_query_results = client
        .query(SELECT_QUESTIONS, &[])
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
        .query(SELECT_QUESTION_OPTIONS, &[])
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

/// Inserts the information about the file into the database.
pub async fn insert_file(
    client: &Client,
    original_name: &str,
    file_uuid: &str,
    extension: &str,
    user_id: &i32,
) -> Result<i32, tokio_postgres::Error> {
    let row = client
        .query_one(INSERT_FILES_QUERY, &[&original_name, &file_uuid, &extension, user_id])
        .await?;
    Ok(row.get("id"))
}

pub async fn insert_answer(client: &Client, answer: Answer, file_id: &i32) {
    let Answer {
        text,
        question_id,
        selection,
    } = answer;
    match text {
        Some(text) => {
            let _ = client.execute(INSERT_TEXT_ANSWER_QUERY, &[&file_id, &question_id, &text]).await;
        }
        None => {
            for selected_response in selection {
                let _ = client
                    .execute(INSERT_SELECTION_ANSWER_QUERY, &[&file_id, &answer.question_id, &selected_response])
                    .await;
            }
        }
    }
}

/// Retrieves all possible tags from the database.
pub async fn retrieve_tags(client: &Client) -> Result<Vec<String>, tokio_postgres::Error> {
    Ok(client
        .query(SELECT_DISTINCT_TAGS, &[])
        .await?
        .into_iter()
        .map(|row| row.get("value"))
        .collect())
}

use sha2::Digest;
use sha2::Sha256;

pub async fn check_login(client: &Client, login_details: LoginDetails) -> Result<User, String> {
    let Some(password) = login_details.password else {
        return Err("Missing password field.".to_string());
    };

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed = hex::encode(hasher.finalize());

    match client.query_one(SELECT_USER, &[&login_details.email, &hashed]).await {
        Ok(row) => {
            let user_role: &str = row.get("role");
            let user_role: UserRole = match user_role {
                "admin" => UserRole::Admin,
                _ => panic!("No such role"),
            };
            let user = User {
                id: row.get("id"),
                role: user_role,
            };
            Ok(user)
        }
        Err(error) => Err(error.to_string()),
    }
}

pub async fn insert_new_password(client: &Client, login_details: LoginDetails) -> Result<(), String> {
    let Some(password) = login_details.password else {
        return Err("Missing password field.".to_string());
    };
    let user = client
        .query_one("select last_password_change as date from users where email = $1", &[&login_details.email])
        .await;

    match user {
        Ok(row) => {
            let date: Option<NaiveDate> = row.get("date");
            let now = Local::now().date_naive();
            if let Some(date) = date {
                if date >= now {
                    return Err("Too many password reset attempts".to_string());
                }
            }
        }
        Err(error) => {
            return Err(format_args!("Invalid query: {}", error).to_string());
        }
    };
    let mut hasher = Sha256::new();
    hasher.update(password);
    let hashed = hex::encode(hasher.finalize());
    match client.execute(SET_PASSWORD, &[&login_details.email, &hashed]).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}
