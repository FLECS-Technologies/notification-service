use crate::config::NotificationServiceConfig;
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn serialize_level<S>(level: &tracing::log::Level, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Level::from(level).serialize(serializer)
}
fn deserialize_level<'de, D>(deserializer: D) -> Result<tracing::log::Level, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Level::deserialize(deserializer)?.into())
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
enum Level {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<&tracing::log::Level> for Level {
    fn from(value: &tracing::log::Level) -> Self {
        match value {
            tracing::log::Level::Error => Self::Error,
            tracing::log::Level::Warn => Self::Warn,
            tracing::log::Level::Info => Self::Info,
            tracing::log::Level::Debug => Self::Debug,
            tracing::log::Level::Trace => Self::Trace,
        }
    }
}

impl From<Level> for tracing::log::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => Self::Error,
            Level::Warn => Self::Warn,
            Level::Info => Self::Info,
            Level::Debug => Self::Debug,
            Level::Trace => Self::Trace,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    #[serde(
        serialize_with = "serialize_level",
        deserialize_with = "deserialize_level"
    )]
    #[schemars(with = "Level")]
    pub level: tracing::log::Level,
}

impl Config {
    pub fn example() -> Self {
        Self {
            level: tracing::log::Level::Info,
        }
    }
}

impl NotificationServiceConfig for Config {
    type Patch = Config;

    fn apply_patch(&mut self, patch: Config) {
        self.level = patch.level
    }
}
