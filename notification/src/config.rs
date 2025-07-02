use crate::services::NotificationService as _;
use crate::services::log::Logger;
use crate::services::smtp::MailServer;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub trace_filter: Option<String>,
    pub port: u16,
    pub default_notification_service: Option<String>,
    pub notification_services: HashMap<String, NotificationService>,
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
