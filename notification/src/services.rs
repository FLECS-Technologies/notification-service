use crate::config::NotificationServiceConfig;
use crate::services::log::Logger;
use crate::services::smtp::MailServer;
use schemars::schema_for;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod log;
pub mod smtp;

pub trait NotificationService {
    type Config: NotificationServiceConfig;
    type NotificationOptions: schemars::JsonSchema + DeserializeOwned;
    fn notification_schema() -> schemars::Schema {
        schema_for!(Self::NotificationOptions)
    }

    fn send_notification(
        &self,
        options: Option<Self::NotificationOptions>,
        config: &Self::Config,
        title: &str,
        content: Option<&str>,
    ) -> Result<(), crate::Error>;

    fn send_notification_with_raw_options(
        &self,
        options: Option<serde_json::Value>,
        config: &Self::Config,
        title: &str,
        content: Option<&str>,
    ) -> Result<(), crate::Error> {
        let options = options.map(serde_json::from_value).transpose()?;
        self.send_notification(options, config, title, content)
    }
}

impl NotisNotificationService {
    pub fn type_string(&self) -> String {
        match self {
            Self::LOG(_) => types::LOG,
            Self::SMTP(_) => types::SMTP,
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

    pub fn config_schema(&self) -> schemars::Schema {
        match self {
            Self::SMTP(_) => <MailServer as NotificationService>::Config::schema(),
            Self::LOG(_) => <Logger as NotificationService>::Config::schema(),
        }
    }

    pub fn notification_schema(&self) -> schemars::Schema {
        match self {
            Self::SMTP(_) => <MailServer as NotificationService>::notification_schema(),
            Self::LOG(_) => <Logger as NotificationService>::notification_schema(),
        }
    }

    pub fn config_patch_schema(&self) -> schemars::Schema {
        match self {
            Self::SMTP(_) => <MailServer as NotificationService>::Config::patch_schema(),
            Self::LOG(_) => <Logger as NotificationService>::Config::patch_schema(),
        }
    }
}

pub mod types {
    pub const SMTP: &str = "smtp";
    pub const LOG: &str = "log";
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum NotisNotificationService {
    SMTP(smtp::Config),
    LOG(log::Config),
}
