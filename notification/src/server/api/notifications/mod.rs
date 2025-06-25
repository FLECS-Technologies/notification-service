use notis_server::apis::notifications::NotificationsPostResponse as PostResponse;
use notis_server::models;
use notis_server::models::NotificationsPostRequest as PostRequest;

pub async fn post(request: PostRequest) -> PostResponse {
    let config = crate::config::get();
    match &config
        .default_notification_service
        .as_ref()
        .and_then(|default| config.notification_services.get(default))
    {
        Some(crate::config::NotificationService::LOG(config)) => {
            let logger = crate::services::log::Logger::new(config);
            logger.log(request.title.as_str(), request.content.as_deref());
            PostResponse::Status200_Success
        }
        Some(crate::config::NotificationService::SMTP(config)) => {
            let server = crate::services::smtp::MailServer::new_from_config(config);
            match server.send_mail(&request.title, request.content) {
                Ok(_) => PostResponse::Status200_Success,
                Err(e) => PostResponse::Status500_InternalServerError(
                    models::NotificationsPost500Response {
                        reason: Some(e.to_string()),
                    },
                ),
            }
        }
        None => PostResponse::Status404_NoDefaultServiceFound,
    }
}
