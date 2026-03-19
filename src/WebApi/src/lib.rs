use std::{path::PathBuf, sync::Arc};

use axum::{
    body::{to_bytes, Body},
    extract::State,
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tokio::sync::Mutex;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

pub mod config;
pub mod error;
mod features;
mod logging;
mod shared;
pub mod state;

use state::AppState;

pub fn build_app(home_dir: PathBuf, log_dir: PathBuf, static_dir: Option<PathBuf>) -> Router {
    let state =
        AppState { home_dir, log_dir, log_lock: Arc::new(Mutex::new(())), score_lock: Arc::new(Mutex::new(())) };

    let api = Router::new()
        .route("/api/login", axum::routing::post(features::login::post_login))
        .route("/api/users", get(features::users::get_users))
        .route("/api/dictionary", get(features::training::get_dictionary))
        .route("/api/logs", get(features::logs::get_logs))
        .route("/api/score", get(features::scores::get_score).post(features::scores::post_score))
        .route_layer(middleware::from_fn_with_state(state.clone(), log_api_errors))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let app = Router::new().merge(api);

    match static_dir {
        Some(static_dir) => {
            let index_file = static_dir.join("index.html");

            app.fallback_service(ServeDir::new(static_dir).fallback(ServeFile::new(index_file)))
        }
        None => app,
    }
}

async fn log_api_errors(State(state): State<AppState>, request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let response = next.run(request).await;

    if !response.status().is_client_error() && !response.status().is_server_error() {
        return response;
    }

    let status = response.status();
    let (parts, body) = response.into_parts();
    let body_bytes = match to_bytes(body, 64 * 1024).await {
        Ok(bytes) => bytes,
        Err(error) => format!("failed to read error response body: {error}").into_bytes().into(),
    };
    let message = String::from_utf8_lossy(&body_bytes).into_owned();

    if let Err(error) =
        logging::append_request_error(&state, &method.to_string(), &path, status.as_u16(), &message).await
    {
        eprintln!("failed to write request log: {error}");
    }

    Response::from_parts(parts, Body::from(body_bytes))
}
