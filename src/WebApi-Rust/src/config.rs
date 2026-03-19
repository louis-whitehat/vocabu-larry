use std::{
    env,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use axum_server::tls_rustls::RustlsConfig;
use tokio::fs;

use crate::error::AppError;

const DEFAULT_HTTP_PORT: u16 = 8101;
const DEFAULT_HTTPS_PORT: u16 = 8102;

pub struct ServerConfig {
    pub home_dir: PathBuf,
    pub http_addr: SocketAddr,
    pub https_addr: SocketAddr,
}

impl ServerConfig {
    pub fn from_environment() -> Result<Self, AppError> {
        let home_dir = resolve_home_dir()?;
        let http_port = read_port("VOCABULARRY_HTTP_PORT", DEFAULT_HTTP_PORT)?;
        let https_port = read_port("VOCABULARRY_HTTPS_PORT", DEFAULT_HTTPS_PORT)?;

        Ok(Self {
            home_dir,
            http_addr: SocketAddr::from(([0, 0, 0, 0], http_port)),
            https_addr: SocketAddr::from(([0, 0, 0, 0], https_port)),
        })
    }
}

pub async fn load_tls_config(home_dir: &Path) -> Result<Option<RustlsConfig>, AppError> {
    let cert_path = home_dir.join("selfsigned.crt");
    let key_path = home_dir.join("selfsigned.key");

    if !fs::try_exists(&cert_path).await? || !fs::try_exists(&key_path).await? {
        return Ok(None);
    }

    Ok(Some(
        RustlsConfig::from_pem_file(cert_path, key_path)
            .await
            .map_err(|error| AppError::bad_request(error.to_string()))?,
    ))
}

fn resolve_home_dir() -> Result<PathBuf, AppError> {
    let raw_home = if is_production() {
        env::var("VOCABULARRY_HOME")
            .map_err(|_| AppError::bad_request("VOCABULARRY_HOME must be set when NODE_ENV=production"))?
    } else {
        "../../".to_owned()
    };

    Ok(PathBuf::from(raw_home))
}

fn is_production() -> bool {
    matches!(env::var("NODE_ENV").as_deref(), Ok("production"))
}

fn read_port(name: &str, default: u16) -> Result<u16, AppError> {
    match env::var(name) {
        Ok(value) => value
            .parse::<u16>()
            .map_err(|_| AppError::InvalidConfiguration(format!("{name} must be a valid port number"))),
        Err(_) => Ok(default),
    }
}
