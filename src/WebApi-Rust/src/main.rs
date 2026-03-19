use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

mod config;
mod error;
mod handlers;
mod state;
mod storage;
mod types;

use crate::{config::ServerConfig, error::AppError, state::AppState};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let server_config = ServerConfig::from_environment()?;
    let state = AppState {
        home_dir: server_config.home_dir.clone(),
        score_lock: Arc::new(Mutex::new(())),
    };

    let app = Router::new()
        .route("/api/users", get(handlers::get_users))
        .route("/api/dictionary", get(handlers::get_dictionary))
        .route("/api/score", get(handlers::get_score).post(handlers::post_score))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let http_listener = tokio::net::TcpListener::bind(server_config.http_addr)
        .await
        .map_err(AppError::from)?;

    println!("Listening for HTTP traffic at http://{}", server_config.http_addr);

    if let Some(tls_config) = config::load_tls_config(&server_config.home_dir).await? {
        println!(
            "Listening for HTTPS traffic at https://{}",
            server_config.https_addr
        );

        let http_server = axum::serve(http_listener, app.clone().into_make_service());
        let https_server = axum_server::bind_rustls(server_config.https_addr, tls_config)
            .serve(app.into_make_service());

        tokio::try_join!(http_server, https_server).map_err(AppError::from)?;
    } else {
        println!(
            "HTTPS disabled for https://{} because certificates are missing",
            server_config.https_addr
        );
        axum::serve(http_listener, app.into_make_service())
            .await
            .map_err(AppError::from)?;
    }

    Ok(())
}
