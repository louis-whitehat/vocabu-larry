use std::{path::PathBuf, sync::Arc};

use axum::{routing::get, Router};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

pub mod config;
pub mod error;
mod features;
mod shared;
pub mod state;

use state::AppState;

pub fn build_app(home_dir: PathBuf) -> Router {
    let state = AppState { home_dir, score_lock: Arc::new(Mutex::new(())) };

    Router::new()
        .route("/api/users", get(features::users::get_users))
        .route("/api/dictionary", get(features::training::get_dictionary))
        .route("/api/score", get(features::scores::get_score).post(features::scores::post_score))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
