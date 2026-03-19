use std::{path::PathBuf, sync::Arc};

use axum::{routing::get, Router};
use tokio::sync::Mutex;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

pub mod config;
pub mod error;
mod features;
mod shared;
pub mod state;

use state::AppState;

pub fn build_app(home_dir: PathBuf, static_dir: Option<PathBuf>) -> Router {
    let state = AppState { home_dir, score_lock: Arc::new(Mutex::new(())) };

    let app = Router::new()
        .route("/api/users", get(features::users::get_users))
        .route("/api/dictionary", get(features::training::get_dictionary))
        .route("/api/score", get(features::scores::get_score).post(features::scores::post_score))
        .layer(CorsLayer::permissive())
        .with_state(state);

    match static_dir {
        Some(static_dir) => {
            let index_file = static_dir.join("index.html");

            app.fallback_service(ServeDir::new(static_dir).fallback(ServeFile::new(index_file)))
        }
        None => app,
    }
}
