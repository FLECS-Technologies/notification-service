use crate::services::NotificationService as _;
use crate::services::log::Logger;
use crate::services::smtp::MailServer;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const CONFIG_PATH_ENV: &str = "NOTIS_CONFIG_PATH";
const DEFAULT_PORT: u16 = 15825;
pub fn config_path() -> PathBuf {
    PathBuf::from(
        std::env::var(CONFIG_PATH_ENV)
            .as_deref()
            .unwrap_or_else(|_| panic!("Environment variable {CONFIG_PATH_ENV} is not set")),
    )
}

pub trait NotificationServiceConfig: schemars::JsonSchema {
    type Patch: schemars::JsonSchema;
    fn schema(&self) -> schemars::Schema {
        schema_for!(Self)
    }
    fn patch_schema(&self) -> schemars::Schema {
        schema_for!(Self::Patch)
    }
    fn apply_patch(&mut self, patch: Self::Patch);
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum NotificationService {
    SMTP(crate::services::smtp::Config),
    LOG(crate::services::log::Config),
}

impl NotificationService {
    pub fn type_string(&self) -> String {
        match self {
            Self::LOG(_) => "log",
            Self::SMTP(_) => "smtp",
        }
        .to_string()
    }

    pub fn send_notification_with_raw_options(
        &self,
        options: Option<serde_json::Value>,
        title: &str,
        content: Option<&str>,
    ) -> Result<(), crate::Error> {
        match self {
            Self::SMTP(config) => {
                MailServer.send_notification_with_raw_options(options, config, title, content)
            }
            Self::LOG(config) => {
                Logger.send_notification_with_raw_options(options, config, title, content)
            }
        }
    }

    pub fn send_notification(
        &self,
        title: &str,
        content: Option<&str>,
    ) -> Result<(), crate::Error> {
        match self {
            Self::SMTP(config) => MailServer.send_notification(None, config, title, content),
            Self::LOG(config) => Logger.send_notification(None, config, title, content),
        }
    }
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
    pub notification_services: HashMap<String, NotificationService>,
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
        Self {
            trace_filter: Some("debug".to_string()),
            port: 80,
            default_notification_service: Some("smtp".to_string()),
            notification_services: HashMap::from([
                (
                    "smtp".to_string(),
                    NotificationService::SMTP(crate::services::smtp::Config::example()),
                ),
                (
                    "log".to_string(),
                    NotificationService::LOG(crate::services::log::Config::example()),
                ),
            ]),
        }
    }
}
