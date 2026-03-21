use std::{
    env,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

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
        let binary_path = backend_binary_path(repo_root)?;

        tokio::fs::create_dir_all(&log_dir).await?;
        tokio::fs::create_dir_all(&dictionaries_dir).await?;

        let mut child = Command::new(&binary_path)
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

fn backend_binary_path(repo_root: &Path) -> Result<PathBuf> {
    if let Ok(path) = env::var("ACCEPTANCE_BACKEND_BIN") {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Ok(path);
        }

        return Err(anyhow!(
            "configured backend binary was not found at {}",
            path.display()
        ));
    }

    let executable_name = if cfg!(windows) {
        "vocabu-larry-api.exe"
    } else {
        "vocabu-larry-api"
    };

    let path = repo_root
        .join("tests")
        .join("target")
        .join("acceptance-backend")
        .join("debug")
        .join(executable_name);

    if path.is_file() {
        Ok(path)
    } else {
        Err(anyhow!(
            "backend binary was not found at {}. Build it before running the acceptance harness.",
            path.display()
        ))
    }
}
