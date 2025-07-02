use crate::config::NotificationServiceConfig;
use schemars::schema_for;
use serde::de::DeserializeOwned;

pub mod log;
pub mod smtp;

pub trait NotificationService {
    type Config: NotificationServiceConfig;
    type NotificationOptions: schemars::JsonSchema + DeserializeOwned;
    fn notification_schema(&self) -> schemars::Schema {
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
        let options = options
            .map(serde_json::from_value)
            .transpose()?;
        self.send_notification(options, config, title, content)
    }
}
