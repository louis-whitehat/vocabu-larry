use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserEntry {
    pub name: String,
    pub dictionaries: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryQuery {
    pub user: String,
    pub dictionary: String,
}

#[derive(Debug, Deserialize)]
pub struct ScoreQuery {
    pub user: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreRequest {
    pub user: String,
    pub dictionary: String,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ScoreEntry {
    pub total: u64,
    pub correct: u64,
}

pub type ScoreStore = BTreeMap<String, BTreeMap<String, ScoreEntry>>;