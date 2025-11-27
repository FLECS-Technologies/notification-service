use crate::services::NotisNotificationService;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const CONFIG_PATH_ENV: &str = "NOTIS_CONFIG_PATH";
const DEFAULT_PORT: u16 = 80;
pub fn config_path() -> PathBuf {
    PathBuf::from(
        std::env::var(CONFIG_PATH_ENV)
            .as_deref()
            .unwrap_or_else(|_| panic!("Environment variable {CONFIG_PATH_ENV} is not set")),
    )
}

pub trait NotificationServiceConfig: schemars::JsonSchema {
    type Patch: schemars::JsonSchema;
    fn schema() -> schemars::Schema {
        schema_for!(Self)
    }
    fn patch_schema() -> schemars::Schema {
        schema_for!(Self::Patch)
    }
    fn apply_patch(&mut self, patch: Self::Patch);
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Format(#[from] serde_json::Error),
}

pub fn from_file<P: AsRef<Path>>(path: &P) -> Result<Config, Error> {
    let file = std::fs::File::open(path)?;
    let config = serde_json::from_reader(file)?;
    Ok(config)
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Config {
    pub trace_filter: Option<String>,
    pub port: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_notification_service: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub notification_services: HashMap<String, NotisNotificationService>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: DEFAULT_PORT,
            notification_services: Default::default(),
            default_notification_service: Default::default(),
            trace_filter: Default::default(),
        }
    }
}

impl Config {
    pub fn example() -> Self {
        let smtp =
            NotisNotificationService::SMTP(Box::new(crate::services::smtp::Config::example()));
        let log = NotisNotificationService::LOG(Box::new(crate::services::log::Config::example()));
        Self {
            trace_filter: Some("debug".to_string()),
            port: DEFAULT_PORT,
            default_notification_service: Some(smtp.type_string()),
            notification_services: HashMap::from([
                (smtp.type_string(), smtp),
                (log.type_string(), log),
            ]),
        }
    }
}
