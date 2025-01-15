use tokio_postgres::Client;

pub const INSERT_FILES_QUERY: &'static str = r#"insert into files (original_name, name, type, submitted_by) values ($1, $2, $3, $4) returning id"#;
pub const INSERT_SELECTION_ANSWER_QUERY: &'static str = r#"insert into answers_selection (file_id, question_id, answer_id) values ($1, $2, $3)"#;
pub const INSERT_TEXT_ANSWER_QUERY: &'static str = r#"insert into answers_text (file_id, question_id, text) values ($1, $2, $3)"#;
pub const SELECT_QUESTIONS: &'static str = "select * from questions";
pub const SELECT_QUESTION_OPTIONS: &'static str = "select * from question_options";

const TABLES_SETUP: &'static str = r#"
create table if not exists users
(
    id        serial primary key,
    email     varchar(100),
    password  varchar(150),
    joined_on date default now(),
    role      int  default 0
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

pub async fn setup_db(client: &Client) {
    if let Err(error) = client.batch_execute(TABLES_SETUP).await {
        panic!("Unable to setup the database tables. {:?}", error);
    }
}
