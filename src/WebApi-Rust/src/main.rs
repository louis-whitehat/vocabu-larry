use vocabu_larry_api::{build_app, config, config::ServerConfig, error::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let server_config = ServerConfig::from_environment()?;
    let app = build_app(server_config.home_dir.clone());

    let http_listener = tokio::net::TcpListener::bind(server_config.http_addr).await.map_err(AppError::from)?;

    println!("Listening for HTTP traffic at http://{}", server_config.http_addr);

    if let Some(tls_config) = config::load_tls_config(&server_config.home_dir).await? {
        println!("Listening for HTTPS traffic at https://{}", server_config.https_addr);

        let http_server = axum::serve(http_listener, app.clone().into_make_service());
        let https_server =
            axum_server::bind_rustls(server_config.https_addr, tls_config).serve(app.into_make_service());

        tokio::try_join!(http_server, https_server).map_err(AppError::from)?;
    } else {
        println!("HTTPS disabled for https://{} because certificates are missing", server_config.https_addr);
        axum::serve(http_listener, app.into_make_service()).await.map_err(AppError::from)?;
    }

    Ok(())
}
