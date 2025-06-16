use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::OnceLock;
static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum NotificationService {
    SMTP(crate::smtp::Config),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] serde_json::Error),
}

pub fn init(config: Config) {
    CONFIG.set(config).expect("Config already initialized");
}

pub fn get() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}

pub fn from_file<P: AsRef<Path>>(path: &P) -> Result<Config, Error> {
    let file = std::fs::File::open(path)?;
    let config = serde_json::from_reader(file)?;
    Ok(config)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub trace_filter: Option<String>,
    pub port: u16,
    pub notification_service: NotificationService,
}

impl Config {
    pub fn example() -> Self {
        Self {
            trace_filter: Some("debug".to_string()),
            port: 80,
            notification_service: NotificationService::SMTP(crate::smtp::Config::example()),
        }
    }
}
