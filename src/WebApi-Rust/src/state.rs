use std::{path::PathBuf, sync::Arc};

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub home_dir: PathBuf,
    pub score_lock: Arc<Mutex<()>>,
}