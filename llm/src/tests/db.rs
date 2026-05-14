// use std::collections::HashMap;

// use postgres::{Client, Error, NoTls};
// use serde::{Deserialize, Serialize};

// use crate::server::build_db_client;

// use super::super::server::retrieve_questions;

// #[test]
// fn connect() {
//     let _ = build_db_client();
// }

// #[test]
// fn insert_question() {
//     let mut client = build_db_client();
//     client
//         .execute(
//             r#"
//             insert into questions (title, text, type)
//             values ($1, $2, $3)
//             "#,
//             &[
//                 &"Description",
//                 &"Please describe the contents of the file in a few short sentences.",
//                 &0i32,
//             ],
//         )
//         .expect("Unable to insert question");
// }

// #[test]
// fn insert_text_answer() {
//     let mut client = build_db_client();
//     client
//         .execute(
//             r#"
//             insert into text_answers (question_id, text)
//             values ($1, $2)
//             "#,
//             &[&1i32, &"This file contains metrics and bananas."],
//         )
//         .expect("Unable to insert question");
// }

// #[test]
// fn fetch_questions() {
//     let mut client = build_db_client();
//     let questions = retrieve_questions(&mut client);
//     println!(
//         "{}",
//         serde_json::to_string_pretty(&questions).expect("Json should work.")
//     );
// }
