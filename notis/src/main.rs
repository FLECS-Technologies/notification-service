use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{info, warn};

const CONFIG_PATH: &str = "./config.json";

fn init() -> anyhow::Result<()> {
    let config_path = PathBuf::from(CONFIG_PATH);
    let config_exists = config_path.try_exists().unwrap_or(false);

    let config = if config_exists {
        notification::config::from_file(&config_path)?
    } else {
        notification::config::Config::default()
    };

    notification::config::init(config);
    notification::tracing::init();

    if !config_exists {
        warn!("Using default config, as {config_path:?} does not exist");
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct AddRequestBody {
    left: String,
    right: String,
}

async fn add_handler(
    axum::extract::Query(query): axum::extract::Query<HashMap<String, String>>,
    axum::extract::Json(data): axum::extract::Json<AddRequestBody>,
) -> impl axum::response::IntoResponse {
    let radix = match query.get("radix").map(|radix| u32::from_str(radix)) {
        None => notification::config::get().default_radix,
        Some(Ok(radix)) => radix,
        Some(Err(e)) => {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Invalid query parameter 'radix': {e}"),
            );
        }
    };
    match notification::try_add_strs(&data.left, &data.right, radix) {
        Ok(result) => (axum::http::StatusCode::OK, format!("{result}")),
        Err(e) => (axum::http::StatusCode::BAD_REQUEST, format!("{e}")),
    }
}

async fn serve() -> anyhow::Result<()> {
    let app = axum::Router::new().route("/add", axum::routing::post(add_handler));
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
