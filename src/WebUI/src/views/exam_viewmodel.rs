use std::collections::HashMap;
use std::future::Future;

use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub translation: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExamViewModel {
    entries: Option<Vec<DictionaryEntry>>,
    current_entry: Option<DictionaryEntry>,
    your_answer: Option<String>,
    previous_correct: Option<String>,
    answer_correct: Option<bool>,
    total_count: u32,
    correct_count: u32,
    failure_counts: HashMap<String, u32>,
    error_message: Option<String>,
}

impl ExamViewModel {
    pub fn loading() -> Self {
        Self {
            entries: None,
            current_entry: None,
            your_answer: None,
            previous_correct: None,
            answer_correct: None,
            total_count: 0,
            correct_count: 0,
            failure_counts: HashMap::new(),
            error_message: None,
        }
    }

    pub fn loaded(entries: Vec<DictionaryEntry>, random_factor: f64) -> Self {
        let failure_counts = HashMap::new();
        let current_entry = select_weighted_entry(&entries, &failure_counts, random_factor);

        Self {
            entries: Some(entries),
            current_entry,
            your_answer: None,
            previous_correct: None,
            answer_correct: None,
            total_count: 0,
            correct_count: 0,
            failure_counts,
            error_message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            entries: None,
            current_entry: None,
            your_answer: None,
            previous_correct: None,
            answer_correct: None,
            total_count: 0,
            correct_count: 0,
            failure_counts: HashMap::new(),
            error_message: Some(message.into()),
        }
    }

    pub async fn load_with<F, Fut>(loader: F, random_factor: f64) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Vec<DictionaryEntry>, String>>,
    {
        match loader().await {
            Ok(entries) => Self::loaded(entries, random_factor),
            Err(error) => Self::error(error),
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn is_loading(&self) -> bool {
        self.entries.is_none() && self.error_message.is_none()
    }

    pub fn current_word(&self) -> Option<&str> {
        self.current_entry.as_ref().map(|entry| entry.word.as_str())
    }

    pub fn current_translation_word_count(&self) -> usize {
        self.current_entry
            .as_ref()
            .map(|entry| entry.translation.split_whitespace().count())
            .unwrap_or(0)
    }

    pub fn answer_correct(&self) -> Option<bool> {
        self.answer_correct
    }

    pub fn your_answer(&self) -> Option<&str> {
        self.your_answer.as_deref()
    }

    pub fn previous_correct(&self) -> Option<&str> {
        self.previous_correct.as_deref()
    }

    pub fn correct_count(&self) -> u32 {
        self.correct_count
    }

    pub fn total_count(&self) -> u32 {
        self.total_count
    }

    pub async fn submit_answer_with<F, Fut>(
        &mut self,
        given_answer: String,
        user: String,
        dictionary: String,
        random_factor: f64,
        record_score: F,
    ) -> Result<(), String>
    where
        F: FnOnce(String, String, bool) -> Fut,
        Fut: Future<Output = Result<(), String>>,
    {
        let Some(entry) = self.current_entry.clone() else {
            return Ok(());
        };

        let is_correct = answers_match(&entry.translation, &given_answer, &dictionary);
        self.previous_correct = Some(entry.translation.clone());
        self.your_answer = Some(given_answer);
        self.answer_correct = Some(is_correct);
        self.total_count += 1;

        if is_correct {
            self.correct_count += 1;
        } else {
            let entry_key = get_entry_key(&entry);
            let failures = self.failure_counts.get(&entry_key).copied().unwrap_or(0);
            self.failure_counts.insert(entry_key, failures + 1);
        }

        self.current_entry = self
            .entries
            .as_deref()
            .and_then(|entries| select_weighted_entry(entries, &self.failure_counts, random_factor));

        record_score(user, dictionary, is_correct).await
    }
}

fn get_entry_key(entry: &DictionaryEntry) -> String {
    format!("{}\n{}", entry.word, entry.translation)
}

fn get_entry_weight(entry: &DictionaryEntry, failure_counts: &HashMap<String, u32>) -> u32 {
    1 + failure_counts
        .get(&get_entry_key(entry))
        .copied()
        .unwrap_or(0)
}

fn select_weighted_entry(
    entries: &[DictionaryEntry],
    failure_counts: &HashMap<String, u32>,
    random_factor: f64,
) -> Option<DictionaryEntry> {
    if entries.is_empty() {
        return None;
    }

    let total_weight: u32 = entries
        .iter()
        .map(|entry| get_entry_weight(entry, failure_counts))
        .sum();
    let bounded_random = if random_factor.is_finite() {
        random_factor.clamp(0.0, 0.999_999_999_999_999_9)
    } else {
        0.0
    };
    let mut remaining_weight = bounded_random * f64::from(total_weight);

    for entry in entries {
        remaining_weight -= f64::from(get_entry_weight(entry, failure_counts));
        if remaining_weight < 0.0 {
            return Some(entry.clone());
        }
    }

    entries.last().cloned()
}

fn normalize_answer(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn strip_leading_token(value: &str, token: &str) -> String {
    if value.starts_with(&format!("{token} ")) {
        value[token.len() + 1..].to_owned()
    } else {
        value.to_owned()
    }
}

fn get_dictionary_language(dictionary_name: &str) -> Option<&str> {
    let language = dictionary_name.split('.').next()?.trim().to_lowercase();

    match language.as_str() {
        "english" => Some("english"),
        "french" => Some("french"),
        _ => None,
    }
}

fn answers_match(expected_answer: &str, actual_answer: &str, dictionary_name: &str) -> bool {
    let normalized_expected = normalize_answer(expected_answer);
    let normalized_actual = normalize_answer(actual_answer);

    if normalized_expected == normalized_actual {
        return true;
    }

    if get_dictionary_language(dictionary_name) != Some("english") {
        return false;
    }

    strip_leading_token(&normalized_expected, "to") == strip_leading_token(&normalized_actual, "to")
        || strip_leading_token(&normalized_expected, "the")
            == strip_leading_token(&normalized_actual, "the")
}