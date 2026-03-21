use std::future::Future;

use serde::{Deserialize, Serialize};

pub fn users_path() -> &'static str {
    "/api/users"
}

pub fn login_path() -> &'static str {
    "/api/login"
}

pub fn login_request(user: impl Into<String>) -> LoginRequest {
    LoginRequest { user: user.into() }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct UserEntry {
    pub name: String,
    pub dictionaries: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub user: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoginViewModel {
    users: Option<Vec<UserEntry>>,
    selected_user: Option<String>,
    selected_dictionary: Option<String>,
    last_logged_user: Option<String>,
    error_message: Option<String>,
}

impl LoginViewModel {
    pub fn loading() -> Self {
        Self {
            users: None,
            selected_user: None,
            selected_dictionary: None,
            last_logged_user: None,
            error_message: None,
        }
    }

    pub fn loaded(users: Vec<UserEntry>) -> Self {
        Self {
            users: Some(users),
            selected_user: None,
            selected_dictionary: None,
            last_logged_user: None,
            error_message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            users: None,
            selected_user: None,
            selected_dictionary: None,
            last_logged_user: None,
            error_message: Some(message.into()),
        }
    }

    pub async fn load_with<F, Fut>(loader: F) -> Self
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Vec<UserEntry>, String>>,
    {
        match loader().await {
            Ok(users) => Self::loaded(users),
            Err(error) => Self::error(error),
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn is_loading(&self) -> bool {
        self.users.is_none() && self.error_message.is_none()
    }

    pub fn users(&self) -> &[UserEntry] {
        self.users.as_deref().unwrap_or(&[])
    }

    pub fn selected_user(&self) -> Option<&str> {
        self.selected_user.as_deref()
    }

    pub fn selected_dictionary(&self) -> Option<&str> {
        self.selected_dictionary.as_deref()
    }

    pub fn dictionaries(&self) -> Vec<String> {
        let Some(selected_user) = &self.selected_user else {
            return Vec::new();
        };

        self.users()
            .iter()
            .find(|entry| &entry.name == selected_user)
            .map(|entry| entry.dictionaries.clone())
            .unwrap_or_default()
    }

    pub fn select_user(&mut self, next_user: Option<String>) -> Option<String> {
        self.selected_dictionary = None;
        self.selected_user = next_user.clone();

        let should_log = next_user.is_some() && next_user != self.last_logged_user;
        if should_log {
            next_user
        } else {
            None
        }
    }

    pub fn mark_user_logged(&mut self, user: String) {
        self.last_logged_user = Some(user);
        self.error_message = None;
    }

    pub fn set_error_message(&mut self, message: impl Into<String>) {
        self.error_message = Some(message.into());
    }

    pub fn select_dictionary(&mut self, next_dictionary: Option<String>) {
        self.selected_dictionary = next_dictionary;
    }
}