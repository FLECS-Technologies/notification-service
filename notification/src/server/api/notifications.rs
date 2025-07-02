use notis_server::apis::notifications::NotificationsPostResponse as PostResponse;
use notis_server::models;
use notis_server::models::NotificationsPostRequest as PostRequest;

pub fn post(config: &crate::config::Config, request: PostRequest) -> PostResponse {
    match &config
        .default_notification_service
        .as_ref()
        .and_then(|default| config.notification_services.get(default))
    {
        Some(service) => {
            match service.send_notification(&request.title, request.content.as_deref()) {
                Ok(_) => PostResponse::Status200_Success,
                Err(e) => {
                    PostResponse::Status500_InternalServerError(models::ServicesIdPut400Response {
                        reason: Some(e.to_string()),
                    })
                }
            }
        }
        None => PostResponse::Status404_NoDefaultServiceFound,
    }
}
