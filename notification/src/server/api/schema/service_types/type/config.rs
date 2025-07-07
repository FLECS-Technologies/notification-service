use crate::config::NotificationServiceConfig;
use crate::services;
use notis_server::apis::services::SchemaServiceTypesTypeConfigGetResponse as GetResponse;
use notis_server::models::SchemaServiceTypesTypeConfigGetPathParams as GetPathParams;
use notis_server::types;

pub fn get(path_params: GetPathParams) -> GetResponse {
    let schema = match path_params.r#type.as_str() {
        "log" => services::log::Config::schema(),
        "smtp" => services::smtp::Config::schema(),
        _ => return GetResponse::Status404_ServiceTypeNotFound,
    };
    GetResponse::Status200_Success(types::Object(serde_json::to_value(schema).unwrap()))
}
