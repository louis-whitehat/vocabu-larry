use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct UserEntry {
    pub name: String,
    pub dictionaries: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub translation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreRequest {
    pub user: String,
    pub dictionary: String,
    pub is_correct: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ScoreEntry {
    pub total: u64,
    pub correct: u64,
}

pub type ScoreStore = BTreeMap<String, BTreeMap<String, ScoreEntry>>;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResponse {
    pub files: Vec<String>,
    pub selected_file: Option<String>,
    pub content: String,
}
