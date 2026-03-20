use std::{
    fmt,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use tempfile::TempDir;

use super::{backend::BackendHandle, browser::BrowserSession};

pub struct AcceptanceWorld {
    repo_root: PathBuf,
    temp_home: Option<TempDir>,
    backend: Option<BackendHandle>,
    browser: Option<BrowserSession>,
}

impl Default for AcceptanceWorld {
    fn default() -> Self {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("tests crate should live under the repository root")
            .to_path_buf();

        Self {
            repo_root,
            temp_home: None,
            backend: None,
            browser: None,
        }
    }
}

impl fmt::Debug for AcceptanceWorld {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcceptanceWorld")
            .field("repo_root", &self.repo_root)
            .field("has_temp_home", &self.temp_home.is_some())
            .field("has_backend", &self.backend.is_some())
            .field("has_browser", &self.browser.is_some())
            .finish()
    }
}

impl AcceptanceWorld {
    pub async fn seed_dictionary(
        &mut self,
        user: &str,
        dictionary: &str,
        content: &str,
    ) -> Result<()> {
        let home = self.ensure_temp_home()?;
        let user_dir = home.path().join("dictionaries").join(user);
        tokio::fs::create_dir_all(&user_dir).await?;
        tokio::fs::write(user_dir.join(format!("{dictionary}.txt")), content).await?;
        Ok(())
    }

    pub async fn seed_log_file(&mut self, file_name: &str, content: &str) -> Result<()> {
        let home = self.ensure_temp_home()?;
        let log_dir = home.path().join("logs");
        tokio::fs::create_dir_all(&log_dir).await?;
        tokio::fs::write(log_dir.join(file_name), content).await?;
        Ok(())
    }

    pub async fn start_application(&mut self) -> Result<()> {
        if self.backend.is_none() {
            let repo_root = self.repo_root.clone();
            let temp_home = self.ensure_temp_home()?;
            self.backend = Some(BackendHandle::start(&repo_root, temp_home.path()).await?);
        }

        if self.browser.is_none() {
            self.browser = Some(BrowserSession::start().await?);
        }

        Ok(())
    }

    pub fn base_url(&self) -> Result<String> {
        self.backend
            .as_ref()
            .map(BackendHandle::base_url)
            .ok_or_else(|| anyhow!("backend was not started"))
    }

    pub fn browser(&self) -> Result<&fantoccini::Client> {
        self.browser
            .as_ref()
            .map(BrowserSession::client)
            .ok_or_else(|| anyhow!("browser was not started"))
    }

    fn ensure_temp_home(&mut self) -> Result<&TempDir> {
        if self.temp_home.is_none() {
            self.temp_home = Some(TempDir::new()?);
        }

        self.temp_home
            .as_ref()
            .ok_or_else(|| anyhow!("temp home should exist after initialization"))
    }
}
