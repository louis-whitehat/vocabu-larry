use std::{path::Path, process::Stdio, time::Duration};

use anyhow::{anyhow, Result};
use portpicker::pick_unused_port;
use reqwest::StatusCode;
use tokio::{
    process::{Child, Command},
    time::sleep,
};

pub struct BackendHandle {
    child: Child,
    port: u16,
}

impl BackendHandle {
    pub async fn start(repo_root: &Path, home_dir: &Path) -> Result<Self> {
        let port = pick_unused_port().ok_or_else(|| anyhow!("failed to allocate an HTTP port"))?;
        let log_dir = home_dir.join("logs");
        let dictionaries_dir = home_dir.join("dictionaries");
        let cargo_target_dir = home_dir.join("backend-target");
        let webapi_dir = repo_root.join("src").join("WebApi");

        tokio::fs::create_dir_all(&log_dir).await?;
        tokio::fs::create_dir_all(&dictionaries_dir).await?;
        tokio::fs::create_dir_all(&cargo_target_dir).await?;

        let mut child = Command::new("cargo")
            .current_dir(&webapi_dir)
            .arg("run")
            .arg("--manifest-path")
            .arg(webapi_dir.join("Cargo.toml"))
            .env("CARGO_TARGET_DIR", &cargo_target_dir)
            .env("NODE_ENV", "production")
            .env("VOCABULARRY_HOME", home_dir)
            .env("VOCABULARRY_LOG_DIR", &log_dir)
            .env("VOCABULARRY_HTTP_PORT", port.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let base_url = format!("http://127.0.0.1:{port}");
        for _ in 0..240 {
            if let Some(status) = child.try_wait()? {
                return Err(anyhow!(
                    "backend exited before becoming ready with status {status}"
                ));
            }

            match reqwest::get(format!("{base_url}/api/users")).await {
                Ok(response) if response.status() == StatusCode::OK => {
                    return Ok(Self { child, port });
                }
                _ => sleep(Duration::from_millis(250)).await,
            }
        }

        let _ = child.kill().await;
        Err(anyhow!("backend did not become ready in time"))
    }

    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }
}

impl Drop for BackendHandle {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}
