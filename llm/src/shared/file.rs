use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::file_type::FileType;

// Represents a single answer to a question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub question_id: i64,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WoodstockFileData {
    pub path: String,
    pub internal_id: i64,
    pub original_name: String,
    pub answers: Vec<Answer>,
    pub tags: Option<Vec<String>>,
    pub file_type: FileType,
    pub submitted_by: i64,
    pub date_of_submission: NaiveDateTime,
}
