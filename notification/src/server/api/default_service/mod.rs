use crate::config::Config;
use notis_server::apis::services::{
    DefaultServiceDeleteResponse as DeleteResponse, DefaultServiceGetResponse as GetResponse,
    DefaultServicePostResponse as PostResponse,
};
use notis_server::models;
use notis_server::models::DefaultServicePostRequest as PostRequest;

pub fn get(config: &Config) -> GetResponse {
    match &config.default_notification_service {
        None => GetResponse::Status404_NoDefaultServiceRegistered,
        Some(id) => match config.notification_services.get(id) {
            None => GetResponse::Status404_NoDefaultServiceRegistered,
            Some(service) => GetResponse::Status200_Success(models::DefaultServiceGet200Response {
                id: id.clone(),
                r#type: service.type_string(),
            }),
        },
    }
}

pub fn post(config: &mut Config, request: PostRequest) -> PostResponse {
    if config.notification_services.contains_key(&request.id) {
        match config.default_notification_service.replace(request.id) {
            None => PostResponse::Status201_DefaultServiceWasSet,
            Some(_) => PostResponse::Status200_DefaultServiceWasReplaced,
        }
    } else {
        PostResponse::Status404_ServiceNotFound
    }
}

pub fn delete(config: &mut Config) -> DeleteResponse {
    match &config.default_notification_service.take() {
        Some(_) => DeleteResponse::Status200_Success,
        None => DeleteResponse::Status404_NoDefaultServiceRegistered,
    }
}
