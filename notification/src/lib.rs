use thiserror::Error;

pub mod config;
pub mod server;
pub mod services;
pub mod tracing;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Smtp(#[from] services::smtp::Error),
}
