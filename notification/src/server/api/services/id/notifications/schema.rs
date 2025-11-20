use crate::config::Config;
use notis_server::apis::services::ServicesIdNotificationsSchemaGetResponse as GetResponse;
use notis_server::models::ServicesIdNotificationsSchemaGetPathParams as GetPathParams;
use notis_server::types;

pub fn get(config: &Config, path_params: GetPathParams) -> GetResponse {
    let schema = match config.notification_services.get(&path_params.id) {
        None => return GetResponse::Status404_ServiceNotFound,
        Some(service) => service.notification_schema(),
    };
    GetResponse::Status200_Success(types::Object(serde_json::to_value(schema).unwrap()))
}
