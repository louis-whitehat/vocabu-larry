use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::Value;
use tokio::fs;
use tower::util::ServiceExt;
use vocabu_larry_api::build_app;

struct TestHome {
    path: PathBuf,
}

impl TestHome {
    fn new() -> Self {
        let unique =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("system time should be after unix epoch").as_nanos();
        let path = std::env::temp_dir().join(format!("vocabu-larry-smoke-{unique}"));

        std::fs::create_dir_all(&path).expect("test home directory should be creatable");

        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestHome {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[tokio::test]
async fn users_endpoint_lists_users_and_dictionaries() {
    let home = TestHome::new();
    seed_dictionary(home.path(), "zoe", "verbs", "go: gehen\n").await;
    seed_dictionary(home.path(), "anna", "animals", "dog: Hund\n").await;
    seed_dictionary(home.path(), "anna", "colors", "red: rot\n").await;

    let response = build_app(home.path().to_path_buf())
        .oneshot(Request::builder().uri("/api/users").body(Body::empty()).expect("request should build"))
        .await
        .expect("router should respond");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.expect("body should be readable");
    let json: Value = serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(json[0]["name"], "anna");
    assert_eq!(json[0]["dictionaries"], serde_json::json!(["animals", "colors"]));
    assert_eq!(json[1]["name"], "zoe");
    assert_eq!(json[1]["dictionaries"], serde_json::json!(["verbs"]));
}

#[tokio::test]
async fn dictionary_endpoint_returns_dictionary_content() {
    let home = TestHome::new();
    seed_dictionary(home.path(), "anna", "animals", "dog: Hund\ncat: Katze\n").await;

    let response = build_app(home.path().to_path_buf())
        .oneshot(
            Request::builder()
                .uri("/api/dictionary?user=anna&dictionary=animals")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("router should respond");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.expect("body should be readable");
    assert_eq!(String::from_utf8(body.to_vec()).expect("body should be utf-8"), "dog: Hund\ncat: Katze\n");
}

#[tokio::test]
async fn score_endpoints_round_trip_basic_progress() {
    let home = TestHome::new();
    let app = build_app(home.path().to_path_buf());

    let initial_response = app
        .clone()
        .oneshot(Request::builder().uri("/api/score?user=anna").body(Body::empty()).expect("request should build"))
        .await
        .expect("router should respond");

    assert_eq!(initial_response.status(), StatusCode::OK);
    let initial_body = to_bytes(initial_response.into_body(), usize::MAX).await.expect("body should be readable");
    assert_eq!(serde_json::from_slice::<Value>(&initial_body).expect("score should be json"), serde_json::json!({}));

    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/score")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "user": "anna",
                        "dictionary": "animals",
                        "isCorrect": true
                    })
                    .to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("router should respond");

    assert_eq!(post_response.status(), StatusCode::OK);

    let read_back_response = app
        .oneshot(Request::builder().uri("/api/score?user=anna").body(Body::empty()).expect("request should build"))
        .await
        .expect("router should respond");

    assert_eq!(read_back_response.status(), StatusCode::OK);

    let body = to_bytes(read_back_response.into_body(), usize::MAX).await.expect("body should be readable");
    let json: Value = serde_json::from_slice(&body).expect("score should be json");
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    assert_eq!(json[&today]["animals"]["total"], 1);
    assert_eq!(json[&today]["animals"]["correct"], 1);
}

#[tokio::test]
async fn invalid_dictionary_path_is_rejected() {
    let home = TestHome::new();
    seed_dictionary(home.path(), "anna", "animals", "dog: Hund\n").await;

    let response = build_app(home.path().to_path_buf())
        .oneshot(
            Request::builder()
                .uri("/api/dictionary?user=anna&dictionary=../secret")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("router should respond");

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = to_bytes(response.into_body(), usize::MAX).await.expect("body should be readable");
    assert_eq!(String::from_utf8(body.to_vec()).expect("body should be utf-8"), "invalid dictionary");
}

async fn seed_dictionary(home_dir: &Path, user: &str, dictionary: &str, content: &str) {
    let user_dir = home_dir.join("dictionaries").join(user);
    fs::create_dir_all(&user_dir).await.expect("user directory should be creatable");
    fs::write(user_dir.join(format!("{dictionary}.txt")), content).await.expect("dictionary file should be writable");
}
