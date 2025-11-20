use crate::server::reason;
use crate::services::Attachment;
use models::ServicesIdNotificationsPostPathParams as PostPathParams;
use notis_server::apis::notifications::ServicesIdNotificationsPostResponse as PostResponse;
use notis_server::models;
use std::error::Error;

pub mod schema;

const TITLE: &str = "title";
const ATTACHMENT: &str = "attachment";
const CONFIG: &str = "config";
const CONTENT: &str = "content";

fn to_internal_server_error(error: impl Error) -> PostResponse {
    PostResponse::Status500_InternalServerError(models::Reason {
        reason: Some(error.to_string()),
    })
}
fn to_bad_request(error: impl Error) -> PostResponse {
    bad_request(error.to_string())
}

fn bad_request(reason: String) -> PostResponse {
    PostResponse::Status400_BadRequest(models::Reason {
        reason: Some(reason),
    })
}

pub async fn read_post_request_from_multipart(
    mut multipart: axum_extra::extract::Multipart,
) -> Result<PostRequest, PostResponse> {
    let mut attachments = Vec::new();
    let mut title = None;
    let mut config = None;
    let mut content = None;
    while let Some(part) = multipart
        .next_field()
        .await
        .map_err(to_internal_server_error)?
    {
        match part.name() {
            Some(TITLE) if title.is_some() => {
                return Err(bad_request(format!("Duplicate field {TITLE}")));
            }
            Some(TITLE) => {
                title.replace(part.text().await.map_err(to_bad_request)?);
            }
            Some(CONTENT) if content.is_some() => {
                return Err(bad_request(format!("Duplicate field {CONTENT}")));
            }
            Some(CONTENT) => {
                content.replace(part.text().await.map_err(to_bad_request)?);
            }
            Some(CONFIG) if config.is_some() => {
                return Err(bad_request(format!("Duplicate field {CONFIG}")));
            }
            Some(CONFIG) => {
                config.replace(
                    serde_json::from_str(&part.text().await.map_err(to_bad_request)?)
                        .map_err(to_bad_request)?,
                );
            }
            Some(ATTACHMENT) => {
                let file_name = part
                    .file_name()
                    .ok_or_else(|| {
                        PostResponse::Status400_BadRequest(models::Reason {
                            reason: Some(
                                "Missing file name in Content-Disposition header of attachment"
                                    .to_string(),
                            ),
                        })
                    })?
                    .to_string();
                let content_type = part
                    .content_type()
                    .ok_or_else(|| {
                        PostResponse::Status400_BadRequest(models::Reason {
                            reason: Some("Missing Content-Type header of attachment".to_string()),
                        })
                    })?
                    .parse()
                    .map_err(to_bad_request)?;
                let file_content = part
                    .bytes()
                    .await
                    .map_err(to_internal_server_error)?
                    .to_vec();
                attachments.push(Attachment {
                    content_type,
                    file_name,
                    file_content,
                })
            }
            _ => {}
        }
    }
    Ok(PostRequest {
        config,
        content,
        title: title.ok_or_else(|| {
            PostResponse::Status400_BadRequest(models::Reason {
                reason: Some(format!("Missing {TITLE}")),
            })
        })?,
        attachments,
    })
}

pub struct PostRequest {
    config: Option<serde_json::Value>,
    content: Option<String>,
    title: String,
    attachments: Vec<Attachment>,
}

pub fn post(
    config: &crate::config::Config,
    path_params: PostPathParams,
    request: PostRequest,
) -> Result<PostResponse, PostResponse> {
    let Some(service) = config.notification_services.get(&path_params.id) else {
        return Err(PostResponse::Status404_ServiceNotFound);
    };
    match service.send_notification_with_raw_options(
        request.config,
        request.attachments,
        &request.title,
        request.content.as_deref(),
    ) {
        Ok(_) => Ok(PostResponse::Status200_Success),
        Err(e) => Err(PostResponse::Status500_InternalServerError(reason(e))),
    }
}
