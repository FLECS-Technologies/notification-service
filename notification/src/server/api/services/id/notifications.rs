use crate::server::reason;
use crate::{Error, config};
use notis_server::apis::notifications::ServicesIdNotificationsPostResponse as PostResponse;
use notis_server::models::{
    ServicesIdNotificationsPostPathParams as PostPathParams,
    ServicesIdNotificationsPostRequest as PostRequest,
};

pub mod schema;

pub fn post(
    config: &config::Config,
    path_params: PostPathParams,
    request: PostRequest,
) -> PostResponse {
    let Some(service) = config.notification_services.get(&path_params.id) else {
        return PostResponse::Status404_ServiceNotFound;
    };
    match service.send_notification_with_raw_options(
        request.options.map(|options| options.0),
        request.title.as_str(),
        request.content.as_deref(),
    ) {
        Ok(_) => PostResponse::Status200_Success,
        Err(e @ Error::Serde(_)) => {
            PostResponse::Status400_BadRequest(reason(format!("Invalid options: {e}")))
        }
        Err(e) => PostResponse::Status500_InternalServerError(reason(e.to_string())),
    }
}
