use gloo_net::http::Request;
use js_sys::encode_uri_component;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::{
    DictionaryEntry, LogResponse, LoginRequest, ScoreRequest, ScoreStore, UserEntry,
};

fn api_base() -> String {
    let Some(window) = web_sys::window() else {
        return "http://localhost:8101".to_owned();
    };

    let location = window.location();
    let hostname = location.hostname().unwrap_or_default();
    let port = location.port().unwrap_or_default();
    let host = location.host().unwrap_or_default();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_owned());

    if port == "8101" || port == "8102" {
        format!("{protocol}//{host}")
    } else if hostname == "localhost" || hostname == "127.0.0.1" {
        "http://localhost:8101".to_owned()
    } else {
        format!("{protocol}//{host}")
    }
}

async fn get_json<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    Request::get(&format!("{}{}", api_base(), path))
        .send()
        .await
        .map_err(|error| error.to_string())?
        .json::<T>()
        .await
        .map_err(|error| error.to_string())
}

fn encode_query_value(value: &str) -> String {
    encode_uri_component(value).into()
}

async fn post_json<B>(path: &str, body: &B) -> Result<(), String>
where
    B: Serialize,
{
    Request::post(&format!("{}{}", api_base(), path))
        .header("content-type", "application/json")
        .json(body)
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;

    Ok(())
}

pub async fn fetch_users() -> Result<Vec<UserEntry>, String> {
    get_json("/api/users").await
}

pub async fn post_login(user: String) -> Result<(), String> {
    post_json("/api/login", &LoginRequest { user }).await
}

pub async fn fetch_dictionary(
    user: &str,
    dictionary: &str,
) -> Result<Vec<DictionaryEntry>, String> {
    get_json(&format!(
        "/api/dictionary?user={}&dictionary={}",
        encode_query_value(user),
        encode_query_value(dictionary)
    ))
    .await
}

pub async fn post_score(user: String, dictionary: String, is_correct: bool) -> Result<(), String> {
    post_json(
        "/api/score",
        &ScoreRequest {
            user,
            dictionary,
            is_correct,
        },
    )
    .await
}

pub async fn fetch_scores(user: &str) -> Result<ScoreStore, String> {
    get_json(&format!("/api/score?user={}", encode_query_value(user))).await
}

pub async fn fetch_logs(file: Option<&str>) -> Result<LogResponse, String> {
    match file {
        Some(file) => get_json(&format!("/api/logs?file={}", encode_query_value(file))).await,
        None => get_json("/api/logs").await,
    }
}
