use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub home_dir: PathBuf,
    pub log_dir: PathBuf,
    pub log_lock: Arc<Mutex<()>>,
    pub score_lock: Arc<Mutex<()>>,
}
