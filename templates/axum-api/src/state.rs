//! Application state with typed config.

use crate::config::AppConfig;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}