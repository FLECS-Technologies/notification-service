pub mod id;

use crate::config::Config;
use notis_server::apis::services::ServicesGetResponse as GetResponse;

pub fn get(config: &Config) -> GetResponse {
    GetResponse::Status200_Success(
        config
            .notification_services
            .iter()
            .map(|(id, service)| (id.clone(), service.type_string()))
            .collect(),
    )
}
