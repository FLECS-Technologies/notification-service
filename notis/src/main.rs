use serde::Deserialize;
use std::path::PathBuf;
use tracing::info;

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
        Ok(_) => (axum::http::StatusCode::OK, ""),
        Err(_) => (axum::http::StatusCode::BAD_REQUEST, "Error"),
    }
}

async fn serve() -> anyhow::Result<()> {
    let app = axum::Router::new().route("/notifications/mail", axum::routing::post(mail_handler));
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
