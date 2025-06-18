use axum::extract::{MatchedPath, Request};
use axum::http;
use serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::trace::DefaultOnResponse;
use tracing::{Span, debug_span, info};

const CONFIG_PATH: &str = "./config.json";

fn init() -> anyhow::Result<()> {
    let config_path = PathBuf::from(CONFIG_PATH);
    let config = notification::config::from_file(&config_path).map_err(|e| {
        anyhow::anyhow!(
            "Error reading config: '{e}'\n\
            Make sure there is a valid config at {CONFIG_PATH}, example:\n{}",
            serde_json::to_string_pretty(&notification::config::Config::example()).unwrap()
        )
    })?;

    notification::config::init(config);
    notification::tracing::init();

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct SendMailRequestBody {
    subject: String,
    content: Option<String>,
}

async fn mail_handler(
    axum::extract::Json(data): axum::extract::Json<SendMailRequestBody>,
) -> impl axum::response::IntoResponse {
    let notification::config::NotificationService::SMTP(config) =
        &notification::config::get().notification_service;

    let server = notification::smtp::MailServer::new_from_config(config);
    match server.send_mail(&data.subject, data.content) {
        Ok(_) => (http::StatusCode::OK, String::new()),
        Err(e) => (http::StatusCode::BAD_REQUEST, e.to_string()),
    }
}

async fn serve() -> anyhow::Result<()> {
    let app = axum::Router::new()
        .route("/notifications", axum::routing::post(mail_handler))
        .layer(
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
    let port = notification::config::get().port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!("Listening on 0.0.0.0:{port}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init()?;
    serve().await
}
