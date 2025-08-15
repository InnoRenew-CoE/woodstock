use crate::server::{Answer, LoginDetails, Question, SelectionAnswer, User, UserRole};
use anyhow::bail;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, string};
use tokio_postgres::{Client, NoTls};

const INSERT_LOGIN_STATISTIC: &'static str = r#"insert into logins (user_id) values ($1)"#;
const INSERT_FILES_QUERY: &'static str = r#"insert into files (original_name, name, type, submitted_by) values ($1, $2, $3, $4) returning id"#;
const INSERT_SELECTION_ANSWER_QUERY: &'static str = r#"insert into answers_selection (file_id, question_id, answer_id) values ($1, $2, $3)"#;
const INSERT_TEXT_ANSWER_QUERY: &'static str = r#"insert into answers_text (file_id, question_id, text) values ($1, $2, $3)"#;
const SELECT_USER: &'static str = "select users.id as id, user_roles.title as role from users left join user_roles on users.role = user_roles.id where email = $1 and password = $2";
const SET_PASSWORD: &'static str = "update users set password = $2, last_password_change = now() where email = $1;";
const SELECT_QUESTIONS: &'static str = "select * from questions";
const SELECT_QUESTION_OPTIONS: &'static str = "select * from question_options";
const SELECT_DISTINCT_TAGS: &'static str = "select distinct tag from tags";
const FIND_FILE: &'static str = "select * from files where id = $1;";
const SELECT_UPLOADED_FILES: &'static str = r#"select *
from files
         left join tag_file on tag_file.file_id = files.id
         left join tags on tags.id = tag_file.tag_id
where submitted_by = $1
"#;

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

create table if not exists tags
(
    id  serial primary key,
    tag varchar(50) unique
);

create table if not exists tag_file
(
    id          serial primary key,
    file_id     int references files,
    tag_id int references tags
);

create table if not exists feedback
(
    id           serial primary key,
    submitted_by int references users,
    text         text
);

create table if not exists logins
(
    id   serial primary key,
    user_id int references users,
    time timestamp default now()
);

create table if not exists templates
(
    id                  serial primary key,
    user_id             int references users,
    file                varchar(200),
    submission_date     date default now(),
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

/// Retrieves all possible tags from the database.
pub async fn retrieve_tags(client: &Client) -> Result<Vec<String>, tokio_postgres::Error> {
    Ok(client
        .query(SELECT_DISTINCT_TAGS, &[])
        .await
        .expect("Unable to query tags.")
        .into_iter()
        .map(|row| row.get("tag"))
        .collect())
}

#[derive(Serialize, Debug, Deserialize)]
pub struct UploadedFile {
    name: String,
    date: chrono::NaiveDate,
    file_type: String,
    tags: Vec<String>,
}
/// Retrieves all uploaded files by the user from the database.
pub async fn retrieve_files(user_id: &i32, client: &Client) -> Result<Vec<UploadedFile>, tokio_postgres::Error> {
    let mut files: HashMap<i32, UploadedFile> = HashMap::new();
    client
        .query(SELECT_UPLOADED_FILES, &[user_id])
        .await
        .expect("Unable to query files.")
        .into_iter()
        .for_each(|row| {
            let id: i32 = row.get("id");
            let name: String = row.get("original_name");
            let date: chrono::NaiveDate = row.get("submission_date");
            let file_type: String = row.get("type");
            let tag: Option<String> = row.get("tag");

            let existing = files.entry(id).or_insert(UploadedFile {
                name,
                date,
                file_type,
                tags: vec![],
            });
            if let Some(tag) = tag {
                existing.tags.push(tag);
            }
        });
    Ok(files.into_values().collect())
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

pub async fn insert_answer(client: &Client, answer: Answer, file_id: &i32) -> Result<(), anyhow::Error> {
    let Answer {
        text,
        question_id,
        selection,
        tags,
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
    for tag in tags {
        println!("Looking for a tag: {}", tag);
        let tagRow = match client.query_one("SELECT id from tags where tag = $1 limit 1", &[&tag]).await {
            Ok(row) => Ok(row),
            Err(_) => {
                // insert new tag
                println!("Inserting {tag:?}");
                client.query_one("insert into tags (tag) values ($1) returning id", &[&tag.trim()]).await
            }
        };
        println!("{:?}", tagRow);
        let Ok(row) = tagRow else { bail!("Unable to add tag to the database.") };
        let tag_id: i32 = row.get("id");
        client
            .execute("insert into tag_file (file_id, tag_id) values ($1, $2)", &[file_id, &tag_id])
            .await?;
        // Insert into file_tags relation.
    }
    Ok(())
}

use sha2::{Digest, Sha256};

pub async fn check_login(client: &Client, login_details: LoginDetails) -> Result<User, String> {
    let Some(password) = login_details.password else {
        return Err("Missing password field.".to_string());
    };

    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed = hex::encode(hasher.finalize());

    match client.query_one(SELECT_USER, &[&login_details.email.to_lowercase(), &hashed]).await {
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

            println!("{:?}", client.execute(INSERT_LOGIN_STATISTIC, &[&user.id]).await);
            Ok(user)
        }
        Err(error) => Err(error.to_string()),
    }
}

pub async fn insert_new_password(client: &Client, login_details: &LoginDetails) -> Result<(), String> {
    let Some(password) = login_details.password.clone() else {
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
            return Err(format_args!("User is not on a whitelist: {:?}", error).to_string());
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

pub async fn insert_feedback(feedback: String, user: &i32, client: &mut Client) {
    let _ = client
        .execute("insert into feedback (text, submitted_by) values ($1, $2)", &[&feedback, user])
        .await;
}

pub async fn find_file(document_id: i32, client: &mut Client) -> Result<FileInfo, &str> {
    let row = client.query_one(FIND_FILE, &[&document_id]).await;
    match row {
        Ok(data) => Ok(FileInfo {
            id: data.get("id"),
            internal_id: data.get("internal_id"),
            name: data.get("name"),
            original_name: data.get("original_name"),
            file_type: data.get("type"),
        }),
        Err(error) => {
            eprintln!("Error with finding file: {:?}", error);
            Err("Unable to find the file...")
        }
    }
}

pub async fn insert_template(file_name: String, user_id: &i32, client: &mut Client) -> Result<(), String> {
    let result = client
        .execute("insert into templates (user_id, file) values ($1, $2)", &[user_id, &file_name])
        .await;
    if let Err(error) = result {
        eprintln!("{:?}", error);
        return Err(error.to_string());
    }
    Ok(())
}

#[derive(Debug)]
pub struct FileInfo {
    pub id: i32,
    pub internal_id: Option<String>,
    pub original_name: String,
    pub name: String,
    pub file_type: String,
}
