use async_signal::{Signal, Signals};
use axum::extract::{MatchedPath, Request};
use futures_util::StreamExt;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::DefaultOnResponse;
use tracing::{Span, debug_span, info};

fn init() -> anyhow::Result<notification::config::Config> {
    let config_path = notification::config::config_path();
    let config = match notification::config::from_file(&config_path) {
        Ok(config) => config,
        Err(notification::config::Error::IO(e)) if e.kind() == ErrorKind::NotFound => {
            let config = notification::config::Config::default();
            if let Some(parent) = config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            serde_json::to_writer_pretty(std::fs::File::create_new(&config_path)?, &config)?;
            info!("Created default config file at {config_path:?}, as it did not exist");
            config
        }
        Err(e) => anyhow::bail!(
            "Error reading config: '{e}'\n\
            Make sure there is a valid and accessible config at {config_path:?}, example:\n{}",
            serde_json::to_string_pretty(&notification::config::Config::example())?
        ),
    };

    notification::tracing::init(&config);

    Ok(config)
}

async fn wait_for_shutdown_signal() {
    let mut signals = Signals::new([Signal::Term, Signal::Int]).unwrap();
    info!("Signal handler was initialized");
    while let Some(signal) = signals.next().await {
        info!("Received signal {signal:?}");
        if matches!(signal, Ok(Signal::Int) | Ok(Signal::Term)) {
            break;
        }
    }
}

async fn serve(config: notification::config::Config, config_path: PathBuf) -> anyhow::Result<()> {
    let port = config.port;
    let app = notis_server::server::new(Arc::new(notification::server::Server::new(
        config,
        config_path,
    )));
    let app = app.layer(
        tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                let path = request.uri().path();
                debug_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    path,
                    error = tracing::field::Empty
                )
            })
            .on_request(|_: &Request<_>, _: &Span| tracing::debug!("Received request"))
            .on_failure(
                |error: ServerErrorsFailureClass, _: Duration, span: &Span| {
                    span.record("error", error.to_string());
                },
            )
            .on_response(DefaultOnResponse::default().include_headers(true)),
    );
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("Listening on 0.0.0.0:{port}");
    axum::serve(listener, app)
        .with_graceful_shutdown(wait_for_shutdown_signal())
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = init()?;
    serve(config, notification::config::config_path()).await
}
