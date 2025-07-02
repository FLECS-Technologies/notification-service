pub mod config;
pub mod notifications;

use crate::config::{Config, NotificationService};
use notis_server::apis::services::{
    ServicesIdDeleteResponse as DeleteResponse, ServicesIdGetResponse as GetResponse,
    ServicesIdPutResponse as PutResponse,
};
use notis_server::models;
use notis_server::models::{
    ServicesIdDeletePathParams as DeletePathParams, ServicesIdGetPathParams as GetPathParams,
    ServicesIdPutPathParams as PutPathParams, ServicesIdPutRequest as PutRequest,
};

pub fn delete(config: &mut Config, path_params: DeletePathParams) -> DeleteResponse {
    match config.notification_services.remove(&path_params.id) {
        Some(_) => {
            if config.default_notification_service == Some(path_params.id) {
                config.default_notification_service = None;
            }
            DeleteResponse::Status200_Success
        }
        None => DeleteResponse::Status404_ServiceNotFound,
    }
}

pub fn get(config: &Config, path_params: GetPathParams) -> GetResponse {
    match config.notification_services.get(&path_params.id) {
        Some(service) => GetResponse::Status200_Success(models::ServicesIdGet200Response {
            id: path_params.id,
            r#type: service.type_string(),
        }),
        None => GetResponse::Status404_ServiceNotFound,
    }
}

pub fn put(config: &mut Config, path_params: PutPathParams, request: PutRequest) -> PutResponse {
    let service = match request.r#type.as_str() {
        "smtp" => serde_json::from_value(request.config.0).map(NotificationService::SMTP),
        "log" => serde_json::from_value(request.config.0).map(NotificationService::LOG),
        t => {
            return PutResponse::Status400_BadRequest(models::ServicesIdPut400Response {
                reason: Some(format!("Unknown notification service type '{t}'")),
            });
        }
    };
    match service {
        Ok(service) => {
            if config
                .notification_services
                .insert(path_params.id, service)
                .is_some()
            {
                PutResponse::Status200_ServiceWasReplaced
            } else {
                PutResponse::Status201_ServiceWasCreated
            }
        }
        Err(e) => PutResponse::Status400_BadRequest(models::ServicesIdPut400Response {
            reason: Some(format!("Invalid config: {e}")),
        }),
    }
}
