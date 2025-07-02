use crate::config::{NotificationService, NotificationServiceConfig};
use notis_server::apis::services::ServicesIdConfigSchemaGetResponse as GetResponse;
use notis_server::models::ServicesIdConfigSchemaGetPathParams as GetPathParams;
use notis_server::models::ServicesIdConfigSchemaGetQueryParams as GetQueryParams;
use notis_server::types;

pub fn get(
    config: &crate::config::Config,
    path_params: GetPathParams,
    query_params: GetQueryParams,
) -> GetResponse {
    match &config.notification_services.get(&path_params.id) {
        Some(NotificationService::LOG(config)) => {
            let schema = if query_params.patch.unwrap_or_default() {
                config.patch_schema()
            } else {
                config.schema()
            };
            GetResponse::Status200_Success(types::Object(serde_json::to_value(schema).unwrap()))
        }
        &Some(NotificationService::SMTP(config)) => {
            let schema = if query_params.patch.unwrap_or_default() {
                config.patch_schema()
            } else {
                config.schema()
            };
            GetResponse::Status200_Success(types::Object(serde_json::to_value(schema).unwrap()))
        }
        None => GetResponse::Status404_ServiceNotFound,
    }
}
