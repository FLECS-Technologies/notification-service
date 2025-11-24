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
    A: apis::notifications::Notifications + apis::services::Services + 'static,
{
    // build our application with a route
    Router::new()
        .route(
            "/default_service",
            delete(default_service_delete::<I, A>)
                .get(default_service_get::<I, A>)
                .post(default_service_post::<I, A>),
        )
        .route("/notifications", post(notifications_post::<I, A>))
        .route(
            "/schema/service_types/:service_type/config",
            get(schema_service_types_service_type_config_get::<I, A>),
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
            get(services_id_config_get::<I, A>).patch(services_id_config_patch::<I, A>),
        )
        .route(
            "/services/:id/config/schema",
            get(services_id_config_schema_get::<I, A>),
        )
        .route(
            "/services/:id/notifications",
            post(services_id_notifications_post::<I, A>).layer(DefaultBodyLimit::disable()),
        )
        .route(
            "/services/:id/notifications/schema",
            get(services_id_notifications_schema_get::<I, A>),
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
    body: Multipart,
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
        .services_id_notifications_post(method, host, cookies, path_params, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::notifications::ServicesIdNotificationsPostResponse::Status200_Success
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::notifications::ServicesIdNotificationsPostResponse::Status400_BadRequest
                                                    (body)
                                                => {
                                                  let mut response = response.status(400);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::notifications::ServicesIdNotificationsPostResponse::Status404_ServiceNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::notifications::ServicesIdNotificationsPostResponse::Status500_InternalServerError
                                                    (body)
                                                => {
                                                  let mut response = response.status(500);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
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
fn default_service_delete_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// DefaultServiceDelete - DELETE /default_service
#[tracing::instrument(skip_all)]
async fn default_service_delete<I, A>(
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
    let validation = tokio::task::spawn_blocking(move || default_service_delete_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .default_service_delete(method, host, cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::DefaultServiceDeleteResponse::Status200_Success => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::services::DefaultServiceDeleteResponse::Status404_NoDefaultServiceRegistered => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::services::DefaultServiceDeleteResponse::Status500_InternalServerError(body) => {
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
fn default_service_get_validation() -> std::result::Result<(), ValidationErrors> {
    Ok(())
}
/// DefaultServiceGet - GET /default_service
#[tracing::instrument(skip_all)]
async fn default_service_get<I, A>(
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
    let validation = tokio::task::spawn_blocking(move || default_service_get_validation())
        .await
        .unwrap();

    let Ok(()) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .default_service_get(method, host, cookies)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::DefaultServiceGetResponse::Status200_Success(body) => {
                let mut response = response.status(200);
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
            apis::services::DefaultServiceGetResponse::Status404_NoDefaultServiceRegistered => {
                let mut response = response.status(404);
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct DefaultServicePostBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::DefaultServicePostRequest,
}

#[tracing::instrument(skip_all)]
fn default_service_post_validation(
    body: models::DefaultServicePostRequest,
) -> std::result::Result<(models::DefaultServicePostRequest,), ValidationErrors> {
    let b = DefaultServicePostBodyValidator { body: &body };
    b.validate()?;

    Ok((body,))
}
/// DefaultServicePost - POST /default_service
#[tracing::instrument(skip_all)]
async fn default_service_post<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    State(api_impl): State<I>,
    Json(body): Json<models::DefaultServicePostRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || default_service_post_validation(body))
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
        .default_service_post(method, host, cookies, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::DefaultServicePostResponse::Status200_DefaultServiceWasReplaced => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::services::DefaultServicePostResponse::Status201_DefaultServiceWasSet => {
                let mut response = response.status(201);
                response.body(Body::empty())
            }
            apis::services::DefaultServicePostResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::services::DefaultServicePostResponse::Status500_InternalServerError(body) => {
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
fn schema_service_types_service_type_config_get_validation(
    path_params: models::SchemaServiceTypesServiceTypeConfigGetPathParams,
) -> std::result::Result<
    (models::SchemaServiceTypesServiceTypeConfigGetPathParams,),
    ValidationErrors,
> {
    path_params.validate()?;

    Ok((path_params,))
}
/// SchemaServiceTypesServiceTypeConfigGet - GET /schema/service_types/{service_type}/config
#[tracing::instrument(skip_all)]
async fn schema_service_types_service_type_config_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::SchemaServiceTypesServiceTypeConfigGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        schema_service_types_service_type_config_get_validation(path_params)
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
        .schema_service_types_service_type_config_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::services::SchemaServiceTypesServiceTypeConfigGetResponse::Status200_Success
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::services::SchemaServiceTypesServiceTypeConfigGetResponse::Status404_ServiceTypeNotFound
                                                => {
                                                  let mut response = response.status(404);
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
            apis::services::ServicesGetResponse::Status200_Success(body) => {
                let mut response = response.status(200);
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
fn services_id_config_get_validation(
    path_params: models::ServicesIdConfigGetPathParams,
) -> std::result::Result<(models::ServicesIdConfigGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdConfigGet - GET /services/{id}/config
#[tracing::instrument(skip_all)]
async fn services_id_config_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdConfigGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_id_config_get_validation(path_params))
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
        .services_id_config_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdConfigGetResponse::Status200_Success(body) => {
                let mut response = response.status(200);
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
            apis::services::ServicesIdConfigGetResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ServicesIdConfigPatchBodyValidator<'a> {
    body: &'a crate::types::Object,
}

#[tracing::instrument(skip_all)]
fn services_id_config_patch_validation(
    path_params: models::ServicesIdConfigPatchPathParams,
    body: crate::types::Object,
) -> std::result::Result<
    (
        models::ServicesIdConfigPatchPathParams,
        crate::types::Object,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = ServicesIdConfigPatchBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// ServicesIdConfigPatch - PATCH /services/{id}/config
#[tracing::instrument(skip_all)]
async fn services_id_config_patch<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdConfigPatchPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<crate::types::Object>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_id_config_patch_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_config_patch(method, host, cookies, path_params, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdConfigPatchResponse::Status200_Success => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::services::ServicesIdConfigPatchResponse::Status400_BadRequest(body) => {
                let mut response = response.status(400);
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
            apis::services::ServicesIdConfigPatchResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::services::ServicesIdConfigPatchResponse::Status500_InternalServerError(body) => {
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
fn services_id_config_schema_get_validation(
    path_params: models::ServicesIdConfigSchemaGetPathParams,
    query_params: models::ServicesIdConfigSchemaGetQueryParams,
) -> std::result::Result<
    (
        models::ServicesIdConfigSchemaGetPathParams,
        models::ServicesIdConfigSchemaGetQueryParams,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    query_params.validate()?;

    Ok((path_params, query_params))
}
/// ServicesIdConfigSchemaGet - GET /services/{id}/config/schema
#[tracing::instrument(skip_all)]
async fn services_id_config_schema_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdConfigSchemaGetPathParams>,
    Query(query_params): Query<models::ServicesIdConfigSchemaGetQueryParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        services_id_config_schema_get_validation(path_params, query_params)
    })
    .await
    .unwrap();

    let Ok((path_params, query_params)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_config_schema_get(method, host, cookies, path_params, query_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdConfigSchemaGetResponse::Status200_Success(body) => {
                let mut response = response.status(200);
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
            apis::services::ServicesIdConfigSchemaGetResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
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
            apis::services::ServicesIdDeleteResponse::Status200_Success => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::services::ServicesIdDeleteResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
                response.body(Body::empty())
            }
            apis::services::ServicesIdDeleteResponse::Status500_InternalServerError(body) => {
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
            apis::services::ServicesIdGetResponse::Status200_Success(body) => {
                let mut response = response.status(200);
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
            apis::services::ServicesIdGetResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
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
fn services_id_notifications_schema_get_validation(
    path_params: models::ServicesIdNotificationsSchemaGetPathParams,
) -> std::result::Result<(models::ServicesIdNotificationsSchemaGetPathParams,), ValidationErrors> {
    path_params.validate()?;

    Ok((path_params,))
}
/// ServicesIdNotificationsSchemaGet - GET /services/{id}/notifications/schema
#[tracing::instrument(skip_all)]
async fn services_id_notifications_schema_get<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdNotificationsSchemaGetPathParams>,
    State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation = tokio::task::spawn_blocking(move || {
        services_id_notifications_schema_get_validation(path_params)
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
        .services_id_notifications_schema_get(method, host, cookies, path_params)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdNotificationsSchemaGetResponse::Status200_Success(body) => {
                let mut response = response.status(200);
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
            apis::services::ServicesIdNotificationsSchemaGetResponse::Status404_ServiceNotFound => {
                let mut response = response.status(404);
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

#[derive(validator::Validate)]
#[allow(dead_code)]
struct ServicesIdPutBodyValidator<'a> {
    #[validate(nested)]
    body: &'a models::ServicesIdPutRequest,
}

#[tracing::instrument(skip_all)]
fn services_id_put_validation(
    path_params: models::ServicesIdPutPathParams,
    body: models::ServicesIdPutRequest,
) -> std::result::Result<
    (
        models::ServicesIdPutPathParams,
        models::ServicesIdPutRequest,
    ),
    ValidationErrors,
> {
    path_params.validate()?;
    let b = ServicesIdPutBodyValidator { body: &body };
    b.validate()?;

    Ok((path_params, body))
}
/// ServicesIdPut - PUT /services/{id}
#[tracing::instrument(skip_all)]
async fn services_id_put<I, A>(
    method: Method,
    host: Host,
    cookies: CookieJar,
    Path(path_params): Path<models::ServicesIdPutPathParams>,
    State(api_impl): State<I>,
    Json(body): Json<models::ServicesIdPutRequest>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::services::Services,
{
    #[allow(clippy::redundant_closure)]
    let validation =
        tokio::task::spawn_blocking(move || services_id_put_validation(path_params, body))
            .await
            .unwrap();

    let Ok((path_params, body)) = validation else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
    };

    let result = api_impl
        .as_ref()
        .services_id_put(method, host, cookies, path_params, body)
        .await;

    let mut response = Response::builder();

    let resp = match result {
        Ok(rsp) => match rsp {
            apis::services::ServicesIdPutResponse::Status200_ServiceWasReplaced => {
                let mut response = response.status(200);
                response.body(Body::empty())
            }
            apis::services::ServicesIdPutResponse::Status201_ServiceWasCreated => {
                let mut response = response.status(201);
                response.body(Body::empty())
            }
            apis::services::ServicesIdPutResponse::Status400_BadRequest(body) => {
                let mut response = response.status(400);
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
            apis::services::ServicesIdPutResponse::Status500_InternalServerError(body) => {
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
