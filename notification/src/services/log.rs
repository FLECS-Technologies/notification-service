mod config;

use crate::Error;
use crate::services::{Attachment, NotificationService};
pub use config::Config;
use tracing::log::log;

#[derive(Default)]
pub struct Logger;

impl NotificationService for Logger {
    type Config = Config;
    type NotificationOptions = Config;

    fn send_notification(
        &self,
        options: Option<Self::NotificationOptions>,
        config: &Self::Config,
        title: &str,
        attachments: Vec<Attachment>,
        content: Option<&str>,
    ) -> Result<(), Error> {
        let level = options.map(|config| config.level).unwrap_or(config.level);
        match content {
            Some(content) => log!(level, "{title}: {content}"),
            None => log!(level, "{title}"),
        }
        for attachment in attachments {
            log!(
                level,
                "Attachment {}: type={}, size={}bytes",
                attachment.file_name,
                serde_json::to_string(&attachment.content_type).unwrap_or_default(),
                attachment.file_content.len()
            )
        }
        Ok(())
    }
}
