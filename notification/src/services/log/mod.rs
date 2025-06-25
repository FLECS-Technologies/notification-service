mod config;

pub use config::Config;
use tracing::log::log;

pub struct Logger<'a> {
    config: &'a Config,
}

impl<'a> Logger<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn log(&self, topic: &str, content: Option<&str>) {
        match content {
            Some(content) => log!(self.config.level, "{topic}: {content}"),
            None => log!(self.config.level, "{topic}"),
        }
    }
}
