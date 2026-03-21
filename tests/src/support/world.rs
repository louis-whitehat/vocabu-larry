use std::{
    fmt,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use tempfile::TempDir;
use vocabu_larry_webui::{
    exam_viewmodel::ExamViewModel,
    login_viewmodel::LoginViewModel,
    logs_viewmodel::LogsViewModel,
    score_viewmodel::ScoreViewModel,
};

use super::backend::BackendHandle;

pub struct AcceptanceWorld {
    repo_root: PathBuf,
    temp_home: Option<TempDir>,
    backend: Option<BackendHandle>,
    selected_user: Option<String>,
    selected_dictionary: Option<String>,
    login_view_model: Option<LoginViewModel>,
    exam_view_model: Option<ExamViewModel>,
    logs_view_model: Option<LogsViewModel>,
    score_view_model: Option<ScoreViewModel>,
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
            selected_user: None,
            selected_dictionary: None,
            login_view_model: None,
            exam_view_model: None,
            logs_view_model: None,
            score_view_model: None,
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
            .field("selected_user", &self.selected_user)
            .field("selected_dictionary", &self.selected_dictionary)
            .field("has_login_view_model", &self.login_view_model.is_some())
            .field("has_exam_view_model", &self.exam_view_model.is_some())
            .field("has_logs_view_model", &self.logs_view_model.is_some())
            .field("has_score_view_model", &self.score_view_model.is_some())
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

        Ok(())
    }

    pub fn base_url(&self) -> Result<String> {
        self.backend
            .as_ref()
            .map(BackendHandle::base_url)
            .ok_or_else(|| anyhow!("backend was not started"))
    }

    pub fn set_selected_user(&mut self, user: impl Into<String>) {
        self.selected_user = Some(user.into());
    }

    pub fn selected_user(&self) -> Option<&str> {
        self.selected_user.as_deref()
    }

    pub fn set_selected_dictionary(&mut self, dictionary: impl Into<String>) {
        self.selected_dictionary = Some(dictionary.into());
    }

    pub fn selected_dictionary(&self) -> Option<&str> {
        self.selected_dictionary.as_deref()
    }

    pub fn set_login_view_model(&mut self, view_model: LoginViewModel) {
        self.login_view_model = Some(view_model);
    }

    pub fn login_view_model(&self) -> Result<&LoginViewModel> {
        self.login_view_model
            .as_ref()
            .ok_or_else(|| anyhow!("login view model was not loaded"))
    }

    pub fn set_exam_view_model(&mut self, view_model: ExamViewModel) {
        self.exam_view_model = Some(view_model);
    }

    pub fn exam_view_model(&self) -> Result<&ExamViewModel> {
        self.exam_view_model
            .as_ref()
            .ok_or_else(|| anyhow!("exam view model was not loaded"))
    }

    pub fn set_logs_view_model(&mut self, view_model: LogsViewModel) {
        self.logs_view_model = Some(view_model);
    }

    pub fn logs_view_model(&self) -> Result<&LogsViewModel> {
        self.logs_view_model
            .as_ref()
            .ok_or_else(|| anyhow!("logs view model was not loaded"))
    }

    pub fn set_score_view_model(&mut self, view_model: ScoreViewModel) {
        self.score_view_model = Some(view_model);
    }

    pub fn score_view_model(&self) -> Result<&ScoreViewModel> {
        self.score_view_model
            .as_ref()
            .ok_or_else(|| anyhow!("score view model was not loaded"))
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
