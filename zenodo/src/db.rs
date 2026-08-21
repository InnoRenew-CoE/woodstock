use anyhow::bail;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{env, string};
use tokio_postgres::types::Date;
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
    submitted_by    int references users,
    status          varchar(50) default 'uploaded',
    status_message text default ''
);

alter table files add column if not exists status varchar(50) default 'uploaded';
alter table files add column if not exists status_message text default '';

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
    submission_date     date default now()
);

create table if not exists queries
(
    id          serial primary key,
    user_id     int references users,
    query       text,
    timestamp   timestamp default now()
);

create table if not exists posts
(
    id      serial primary key,
    author  int references users,
    created date default now()
);

create table if not exists post_edits
(
    id      serial primary key,
    editor  int references users,
    time    date default now(),
    post_id int references posts
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

/// Attempts to insert the created query into the database
pub async fn insert_query(user_id: i32, client: &Client, query: &str) -> Result<u64, tokio_postgres::Error> {
    client
        .execute("insert into queries (user_id, query) values ($1, $2)", &[&user_id, &query])
        .await
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

#[derive(Debug)]
pub struct FileInfo {
    pub id: i32,
    pub internal_id: Option<String>,
    pub original_name: String,
    pub name: String,
    pub file_type: String,
}
