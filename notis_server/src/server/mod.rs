use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::notifications::Notifications
        + apis::schema::Schema
        + apis::services::Services
        + 'static,
{
    // build our application with a route
    Router::new()
        .route("/notifications", post(notifications_post::<I, A>))
        .route(
            "/schema/services/:service_type/config",
            get(schema_services_service_type_config_get::<I, A>),
        )
        .route(
            "/schema/services/:service_type/notification",
            get(schema_services_service_type_notification_get::<I, A>),
        )
        .route("/services", get(services_get::<I, A>))
        .route(
            "/services/:id",
            delete(services_id_delete::<I, A>)
                .get(services_id_get::<I, A>)
                .put(services_id_put::<I, A>),
        )
        .route(
            "/services/:id/config",
            patch(services_id_config_patch::<I, A>),
        )
        .route(
            "/services/:id/notifications",
            post(services_id_notifications_post::<I, A>),
        )
        .with_state(api_impl)
}

#[derive(validator::Validate)]
#[allow(dead_code)]
struct NotificationsPostBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::NotificationsPostRequest,
}

#[tracing::instrument(skip_all)]
fn notifications_post_validation(
    body: models::NotificationsPostRequest,
) -> std::result::Result<(models::NotificationsPostRequest,), ValidationErrors> {
    let b = NotificationsPostBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// NotificationsPost - POST /notifications
#[tracing::instrument(skip_all)]
async fn notifications_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::NotificationsPostRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::notifications::Notifications,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || notifications_post_validation(body))
        .await
        .unwrap();

    let Ok((body,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .notifications_post(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::notifications::NotificationsPostResponse::Status200_Success => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::notifications::NotificationsPostResponse::Status404_NoDefaultServiceFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::notifications::NotificationsPostResponse::Status500_InternalServerError(body) => {
                let mut response = response.status(500);
                {
                    let mut response_headers = response.headers_mut().unwrap();
                    response_headers.insert(
                        CONTENT_TYPE,
                        HeaderValue::from_str("application/json").map_err(|e| {
                            error!(error = ?e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?,
                    );
                }

                let body_content = tokio::task::spawn_blocking(move || {
                    serde_json::to_vec(&body).map_err(|e| {
                        error!(error = ?e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })
                })
                .await
                .unwrap()?;
                response.body(Body::from(body_content))
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_id_notifications_post_validation(
    path_params: models::ServicesIdNotificationsPostPathParams,
) -> std::result::Result<(models::ServicesIdNotificationsPostPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdNotificationsPost - POST /services/{id}/notifications
#[tracing::instrument(skip_all)]
async fn services_id_notifications_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdNotificationsPostPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::notifications::Notifications,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_id_notifications_post_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_notifications_post(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::notifications::ServicesIdNotificationsPostResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn schema_services_service_type_config_get_validation(
    path_params: models::SchemaServicesServiceTypeConfigGetPathParams,
) -> std::result::Result<(models::SchemaServicesServiceTypeConfigGetPathParams,), ValidationErrors>
{
    path_params.validate()?;

    Ok((path_params,))
}
/// SchemaServicesServiceTypeConfigGet - GET /schema/services/{service_type}/config
#[tracing::instrument(skip_all)]
async fn schema_services_service_type_config_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::SchemaServicesServiceTypeConfigGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::schema::Schema,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        schema_services_service_type_config_get_validation(path_params)
    })
    .await
    .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .schema_services_service_type_config_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::schema::SchemaServicesServiceTypeConfigGetResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn schema_services_service_type_notification_get_validation(
    path_params: models::SchemaServicesServiceTypeNotificationGetPathParams,
) -> std::result::Result<
    (models::SchemaServicesServiceTypeNotificationGetPathParams,),
    ValidationErrors,
> {
    path_params.validate()?;

    Ok((path_params,))
}
/// SchemaServicesServiceTypeNotificationGet - GET /schema/services/{service_type}/notification
#[tracing::instrument(skip_all)]
async fn schema_services_service_type_notification_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::SchemaServicesServiceTypeNotificationGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::schema::Schema,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        schema_services_service_type_notification_get_validation(path_params)
    })
    .await
    .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .schema_services_service_type_notification_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::schema::SchemaServicesServiceTypeNotificationGetResponse::Status501_NotImplemented
                                                => {
                                                  let mut response = response.status(501);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_get_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// ServicesGet - GET /services
#[tracing::instrument(skip_all)]
async fn services_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || services_get_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl.as_ref().services_get(method, host, cookies).await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesGetResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_id_config_patch_validation(
    path_params: models::ServicesIdConfigPatchPathParams,
) -> std::result::Result<(models::ServicesIdConfigPatchPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdConfigPatch - PATCH /services/{id}/config
#[tracing::instrument(skip_all)]
async fn services_id_config_patch<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdConfigPatchPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_id_config_patch_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_config_patch(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdConfigPatchResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_id_delete_validation(
    path_params: models::ServicesIdDeletePathParams,
) -> std::result::Result<(models::ServicesIdDeletePathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdDelete - DELETE /services/{id}
#[tracing::instrument(skip_all)]
async fn services_id_delete<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdDeletePathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_id_delete_validation(path_params))
            .await
            .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_delete(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdDeleteResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_id_get_validation(
    path_params: models::ServicesIdGetPathParams,
) -> std::result::Result<(models::ServicesIdGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdGet - GET /services/{id}
#[tracing::instrument(skip_all)]
async fn services_id_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || services_id_get_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdGetResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

#[tracing::instrument(skip_all)]
fn services_id_put_validation(
    path_params: models::ServicesIdPutPathParams,
) -> std::result::Result<(models::ServicesIdPutPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdPut - PUT /services/{id}
#[tracing::instrument(skip_all)]
async fn services_id_put<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdPutPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || services_id_put_validation(path_params))
        .await
        .unwrap();

    let Ok((path_params,)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_put(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdPutResponse::Status501_NotImplemented => {
                let mut response = response.status(501);
                response.body(Body::empty())
            }
        },
        Err(_) => {
            // Application code returned an error. This should not happen, as the implementation should
            // return a valid response.
            response.status(500).body(Body::empty())
        }
    };

    resp.map_err(|e| {
        error!(error = ?e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
