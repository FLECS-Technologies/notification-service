pub mod id;

use crate::config::{Config, NotificationService};
use notis_server::apis::services::ServicesGetResponse as GetResponse;

pub fn get(config: &Config) -> GetResponse {
    GetResponse::Status200_Success(
        config
            .notification_services
            .iter()
            .map(|(id, service)| {
                (
                    id.clone(),
                    match service {
                        NotificationService::SMTP(_) => "smtp",
                        NotificationService::LOG(_) => "log",
                    }
                    .to_string(),
                )
            })
            .collect(),
    )
}
