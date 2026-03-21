use std::collections::BTreeMap;
use std::future::Future;

use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ScoreEntry {
    pub total: u64,
    pub correct: u64,
}

pub type ScoreStore = BTreeMap<String, BTreeMap<String, ScoreEntry>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreRow {
    pub date: String,
    pub dictionary: String,
    pub correct: u64,
    pub total: u64,
    pub pass_rate: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScoreViewModel {
    scores: Option<ScoreStore>,
    error_message: Option<String>,
}

impl ScoreViewModel {
    pub fn loading() -> Self {
        Self {
            scores: None,
            error_message: None,
        }
    }

    pub fn loaded(scores: ScoreStore) -> Self {
        Self {
            scores: Some(scores),
            error_message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            scores: None,
            error_message: Some(message.into()),
        }
    }

    pub async fn load_with<F, Fut>(loader: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<ScoreStore, String>>,
    {
        match loader().await {
            Ok(scores) => Self::loaded(scores),
            Err(error) => Self::error(error),
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn is_loading(&self) -> bool {
        self.scores.is_none() && self.error_message.is_none()
    }

    pub fn is_empty(&self) -> bool {
        self.scores.as_ref().is_some_and(BTreeMap::is_empty)
    }

    pub fn rows(&self) -> Vec<ScoreRow> {
        let Some(score_store) = &self.scores else {
            return Vec::new();
        };

        let mut sorted_dates = score_store.keys().cloned().collect::<Vec<_>>();
        sorted_dates.sort_by(|left, right| right.cmp(left));

        sorted_dates
            .into_iter()
            .flat_map(|date| {
                score_store[&date].iter().map(move |(dictionary, stats)| ScoreRow {
                    date: date.clone(),
                    dictionary: dictionary.clone(),
                    correct: stats.correct,
                    total: stats.total,
                    pass_rate: format_pass_rate(stats.correct, stats.total),
                })
            })
            .collect()
    }
}

pub fn format_pass_rate(correct: u64, total: u64) -> String {
    if total == 0 {
        return "0.00".to_owned();
    }

    format!("{:.2}", correct as f64 / total as f64 * 100.0)
}