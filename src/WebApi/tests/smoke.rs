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
async fn smoke_test_training_workflow() {
    let home = TestHome::new();
    seed_dictionary(home.path(), "zoe", "verbs", "go: gehen\n").await;
    seed_dictionary(home.path(), "anna", "animals", "dog: Hund\ncat: Katze\n").await;
    seed_dictionary(home.path(), "anna", "colors", "red: rot\n").await;
    let app = build_app(home.path().to_path_buf(), None);

    let users_response = app_get(&app, "/api/users").await;
    assert_eq!(users_response.status(), StatusCode::OK);

    let users_json = response_json(users_response).await;

    assert_eq!(users_json[0]["name"], "anna");
    assert_eq!(users_json[0]["dictionaries"], serde_json::json!(["animals", "colors"]));
    assert_eq!(users_json[1]["name"], "zoe");
    assert_eq!(users_json[1]["dictionaries"], serde_json::json!(["verbs"]));

    let dictionary_response = app_get(&app, "/api/dictionary?user=anna&dictionary=animals").await;
    assert_eq!(dictionary_response.status(), StatusCode::OK);

    let dictionary_body = response_text(dictionary_response).await;
    assert_eq!(dictionary_body, "dog: Hund\ncat: Katze\n");

    let initial_score_response = app_get(&app, "/api/score?user=anna").await;
    assert_eq!(initial_score_response.status(), StatusCode::OK);
    assert_eq!(response_json(initial_score_response).await, serde_json::json!({}));

    let first_training_response = app_post_json(
        &app,
        "/api/score",
        serde_json::json!({
            "user": "anna",
            "dictionary": "animals",
            "isCorrect": true
        }),
    )
    .await;
    assert_eq!(first_training_response.status(), StatusCode::OK);

    let second_training_response = app_post_json(
        &app,
        "/api/score",
        serde_json::json!({
            "user": "anna",
            "dictionary": "animals",
            "isCorrect": false
        }),
    )
    .await;
    assert_eq!(second_training_response.status(), StatusCode::OK);

    let score_response = app_get(&app, "/api/score?user=anna").await;
    assert_eq!(score_response.status(), StatusCode::OK);

    let score_json = response_json(score_response).await;
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    assert_eq!(score_json[&today]["animals"]["total"], 2);
    assert_eq!(score_json[&today]["animals"]["correct"], 1);
}

async fn app_get(app: &axum::Router, uri: &str) -> axum::response::Response {
    app.clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).expect("request should build"))
        .await
        .expect("router should respond")
}

async fn app_post_json(app: &axum::Router, uri: &str, payload: Value) -> axum::response::Response {
    app.clone()
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .expect("request should build"),
        )
        .await
        .expect("router should respond")
}

async fn response_json(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.expect("body should be readable");
    serde_json::from_slice(&body).expect("response should be valid json")
}

async fn response_text(response: axum::response::Response) -> String {
    let body = to_bytes(response.into_body(), usize::MAX).await.expect("body should be readable");
    String::from_utf8(body.to_vec()).expect("body should be utf-8")
}

async fn seed_dictionary(home_dir: &Path, user: &str, dictionary: &str, content: &str) {
    let user_dir = home_dir.join("dictionaries").join(user);
    fs::create_dir_all(&user_dir).await.expect("user directory should be creatable");
    fs::write(user_dir.join(format!("{dictionary}.txt")), content).await.expect("dictionary file should be writable");
}
