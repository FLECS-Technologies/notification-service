mod config;

use crate::Error;
use crate::services::NotificationService;
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
        content: Option<&str>,
    ) -> Result<(), Error> {
        let level = options.map(|config| config.level).unwrap_or(config.level);
        match content {
            Some(content) => log!(level, "{title}: {content}"),
            None => log!(level, "{title}"),
        }
        Ok(())
    }
}
