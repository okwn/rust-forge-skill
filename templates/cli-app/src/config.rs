use serde::Deserialize;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Log level (trace, debug, info, warn, error).
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Target directory for build artifacts.
    #[serde(default = "default_target_dir")]
    pub target_dir: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_target_dir() -> String {
    "target".to_string()
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// Environment variables:
    /// - `LOG_LEVEL`: Log level (default: "info")
    /// - `TARGET_DIR`: Target directory (default: "target")
    pub fn from_env() -> Self {
        Self {
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| default_log_level()),
            target_dir: std::env::var("TARGET_DIR").unwrap_or_else(|_| default_target_dir()),
        }
    }
}