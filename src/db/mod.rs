use crate::server::{Answer, LoginDetails, Question, SelectionAnswer, User, UserRole};
use anyhow::bail;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::{env, string};
use tokio_postgres::types::Date;
use tokio_postgres::{Client, NoTls};

const INSERT_LOGIN_STATISTIC: &'static str = r#"insert into logins (user_id) values ($1)"#;
/// `internal_id` carries the file uuid, the same value the ingestion pipeline stores as the
/// document id, so `update_file_status` can find the row by it.
const INSERT_FILES_QUERY: &'static str =
    r#"insert into files (original_name, name, type, submitted_by, internal_id) values ($1, $2, $3, $4, $2) returning id"#;
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

const SELECT_STATISTICS_TOTALS: &'static str = r#"
select (select count(*) from files)                                                          as total_files,
       (select count(*) from users)                                                          as total_users,
       (select count(*) from tags)                                                           as total_tags,
       (select count(*) from posts)                                                          as total_posts,
       (select count(*) from feedback)                                                       as total_feedback,
       (select count(*) from queries)                                                        as total_queries,
       (select count(*) from logins)                                                         as total_logins,
       (select (select count(*) from answers_text) + (select count(*) from answers_selection)) as total_answers,
       (select count(*) from files where status = 'done')                                    as processed_files,
       (select count(*) from files where status = 'failed')                                  as failed_files,
       (select count(*) from files where status is null or status not in ('done', 'failed')) as pending_files,
       (select count(distinct submitted_by) from files where submitted_by is not null)       as active_users,
       (select count(*) from files where submission_date >= current_date - interval '7 days') as files_last_week,
       (select count(*) from files where submitted_by = $1)                                  as my_files,
       (select count(*) from files where submitted_by = $1
                                        and submission_date >= date_trunc('month', current_date)) as my_files_this_month,
       (select count(*) from answers_text a join files f on f.id = a.file_id where f.submitted_by = $1)
           + (select count(*) from answers_selection s join files f on f.id = s.file_id where f.submitted_by = $1) as my_answers
"#;

const SELECT_FILE_TYPE_STATISTICS: &'static str = r#"
select coalesce(type, 'unknown') as file_type,
       count(*)                  as count
from files
group by coalesce(type, 'unknown')
order by count(*) desc, file_type asc
"#;

const SELECT_TOP_TAGS: &'static str = r#"
select tags.tag as tag,
       count(*) as count
from tag_file
         join tags on tags.id = tag_file.tag_id
group by tags.tag
order by count(*) desc, tags.tag asc
limit 10
"#;

const SELECT_MONTHLY_UPLOADS: &'static str = r#"
select to_char(m.month_start, 'YYYY-MM') as month,
       (select count(*) from files f where date_trunc('month', f.submission_date) = m.month_start) as count
from generate_series(
             date_trunc('month', current_date) - interval '5 months',
             date_trunc('month', current_date),
             interval '1 month'
     ) as m(month_start)
order by m.month_start
"#;

const SELECT_RECENT_UPLOADS: &'static str = r#"
select coalesce(f.original_name, f.name) as name,
       coalesce(f.type, 'unknown')       as file_type,
       f.submission_date                 as date,
       coalesce(f.status, 'uploaded')    as status,
       u.id                           as submitted_by
from files f
         left join users u on u.id = f.submitted_by
order by f.submission_date desc nulls last, f.id desc
limit 8
"#;

const SELECT_ALL_FILES: &'static str = r#"
select id,
       coalesce(original_name, name, 'unnamed') as name
from files
where submitted_by != 1
order by lower(coalesce(original_name, name)) asc, id asc
"#;

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

-- Backfill the document id for rows created before the insert started writing it,
-- so status updates can match them by internal_id.
update files set internal_id = name where internal_id is null;

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
pub async fn retrieve_tags(client: &Client) -> Result<HashSet<String>, tokio_postgres::Error> {
    Ok(client
        .query(SELECT_DISTINCT_TAGS, &[])
        .await
        .expect("Unable to query tags.")
        .into_iter()
        .map(|row| row.get("tag"))
        .map(|t: String| t.to_lowercase())
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

pub async fn update_file_status(
    client: &Client,
    internal_id: &str,
    status: &str,
    status_message: Option<&str>,
) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "update files set status = $2, status_message = $3 where internal_id = $1",
            &[&internal_id, &status, &status_message.unwrap_or("")],
        )
        .await
}

pub async fn update_file_status_by_id(
    client: &Client,
    file_id: i32,
    status: &str,
    status_message: Option<&str>,
) -> Result<u64, tokio_postgres::Error> {
    client
        .execute(
            "update files set status = $2, status_message = $3 where id = $1",
            &[&file_id, &status, &status_message.unwrap_or("")],
        )
        .await
}

pub async fn insert_answer(client: &Client, answer: &Answer, file_id: &i32) -> Result<(), anyhow::Error> {
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
        let tag = tag.to_lowercase();
        println!("Looking for a tag: {}", tag);
        let tagRow = match client.query_one("SELECT id from tags where tag = $1 limit 1", &[&tag]).await {
            Ok(row) => Ok(row),
            Err(_) => {
                // insert new tag
                println!("Inserting {tag:?}");
                client
                    .query_one("insert into tags (tag) values ($1) returning id", &[&tag.trim().to_lowercase()])
                    .await
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

pub async fn upsert_post(id: Option<i32>, title: String, body: String, user: &i32, client: &mut Client) -> Result<(), &'static str> {
    let upsert_result = if let Some(id) = id {
        client
            .execute(
                r#"
                insert into posts (id, author, title, body)
                values ($1, $2, $3, $4)
                ON CONFLICT (id)
                    DO UPDATE SET title = excluded.title,
                                  body  = excluded.body;
                                  "#,
                &[&id, user, &title, &body],
            )
            .await
    } else {
        client
            .execute(
                r#"
                insert into posts (author, title, body)
                values ($1, $2, $3)
                ON CONFLICT (id)
                    DO UPDATE SET title = excluded.title,
                                  body  = excluded.body;
                                  "#,
                &[user, &title, &body],
            )
            .await
    };
    if let Err(error) = upsert_result {
        eprintln!("{:?}", error);
        return Err("Failed to insert.");
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    email: String,
    created: NaiveDate,
}
pub async fn get_posts(client: &mut Client) -> Result<Vec<Post>, &'static str> {
    let mut vec = Vec::new();
    let rows = client
        .query(
            "SELECT posts.id, posts.title, body, email, created FROM posts left join users on users.id = posts.author",
            &[],
        )
        .await;
    if let Ok(rows) = rows {
        for row in rows {
            let post = Post {
                id: Some(row.get(0)),
                title: row.get(1),
                body: row.get(2),
                email: row.get(3),
                created: row.get(4),
            };
            vec.push(post);
        }
    }

    Ok(vec)
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

#[derive(Debug, Serialize)]
pub struct FileTypeStatistic {
    pub file_type: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct TagStatistic {
    pub tag: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct MonthlyUpload {
    pub month: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct RecentUpload {
    pub name: String,
    pub file_type: String,
    pub date: Option<NaiveDate>,
    pub status: String,
    pub submitted_by: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct Statistics {
    pub total_files: i64,
    pub total_users: i64,
    pub active_users: i64,
    pub total_tags: i64,
    pub total_answers: i64,
    pub total_posts: i64,
    pub total_feedback: i64,
    pub total_queries: i64,
    pub total_logins: i64,
    pub processed_files: i64,
    pub pending_files: i64,
    pub failed_files: i64,
    pub files_last_week: i64,
    pub my_files: i64,
    pub my_files_this_month: i64,
    pub my_answers: i64,
    pub file_types: Vec<FileTypeStatistic>,
    pub top_tags: Vec<TagStatistic>,
    pub monthly_uploads: Vec<MonthlyUpload>,
    pub recent_uploads: Vec<RecentUpload>,
}

/// Returns a percentage of `count` in `total`, rounded to one decimal place.
fn share(count: i64, total: i64) -> f64 {
    if total <= 0 {
        return 0.0;
    }
    (count as f64 * 1000.0 / total as f64).round() / 10.0
}

/// Aggregates corpus, contribution and processing statistics for the dashboard.
pub async fn retrieve_statistics(user_id: &i32, client: &Client) -> Result<Statistics, tokio_postgres::Error> {
    let totals = client.query_one(SELECT_STATISTICS_TOTALS, &[user_id]).await?;
    let total_files: i64 = totals.get("total_files");
    let processed_files: i64 = totals.get("processed_files");
    let failed_files: i64 = totals.get("failed_files");

    let file_types = client
        .query(SELECT_FILE_TYPE_STATISTICS, &[])
        .await?
        .into_iter()
        .map(|row| {
            let count: i64 = row.get("count");
            FileTypeStatistic {
                file_type: row.get("file_type"),
                count,
                percentage: share(count, total_files),
            }
        })
        .collect::<Vec<_>>();

    let top_tags = client
        .query(SELECT_TOP_TAGS, &[])
        .await?
        .into_iter()
        .map(|row| TagStatistic {
            tag: row.get("tag"),
            count: row.get("count"),
        })
        .collect::<Vec<_>>();

    let monthly_uploads = client
        .query(SELECT_MONTHLY_UPLOADS, &[])
        .await?
        .into_iter()
        .map(|row| MonthlyUpload {
            month: row.get("month"),
            count: row.get("count"),
        })
        .collect::<Vec<_>>();

    let recent_uploads = client
        .query(SELECT_RECENT_UPLOADS, &[])
        .await?
        .into_iter()
        .map(|row| RecentUpload {
            name: row.get("name"),
            file_type: row.get("file_type"),
            date: row.get("date"),
            status: row.get("status"),
            submitted_by: row.get("submitted_by"),
        })
        .collect::<Vec<_>>();

    Ok(Statistics {
        total_files,
        total_users: totals.get("total_users"),
        active_users: totals.get("active_users"),
        total_tags: totals.get("total_tags"),
        total_answers: totals.get("total_answers"),
        total_posts: totals.get("total_posts"),
        total_feedback: totals.get("total_feedback"),
        total_queries: totals.get("total_queries"),
        total_logins: totals.get("total_logins"),
        processed_files,
        pending_files: totals.get("pending_files"),
        failed_files,
        files_last_week: totals.get("files_last_week"),
        my_files: totals.get("my_files"),
        my_files_this_month: totals.get("my_files_this_month"),
        my_answers: totals.get("my_answers"),
        file_types,
        top_tags,
        monthly_uploads,
        recent_uploads,
    })
}

#[derive(Debug, Serialize)]
pub struct FileListing {
    pub id: i32,
    pub name: String,
}

/// Retrieves every uploaded file with just the data needed to list and download it.
pub async fn retrieve_all_files(client: &Client) -> Result<Vec<FileListing>, tokio_postgres::Error> {
    Ok(client
        .query(SELECT_ALL_FILES, &[])
        .await?
        .into_iter()
        .map(|row| FileListing {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect())
}
