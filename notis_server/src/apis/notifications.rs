use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum NotificationsPostResponse {
    /// Success
    Status200_Success,
    /// No default service found
    Status404_NoDefaultServiceFound,
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdNotificationsPostResponse {
    /// Success
    Status200_Success,
    /// Bad Request
    Status400_BadRequest(models::Reason),
    /// Service not found
    Status404_ServiceNotFound,
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

/// Notifications
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Notifications {
    /// Send notification via the default service.
    ///
    /// NotificationsPost - POST /notifications
    async fn notifications_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::NotificationsPostRequest,
    ) -> Result<NotificationsPostResponse, ()>;

    /// Send notification via the notification service, may contain additional options.
    ///
    /// ServicesIdNotificationsPost - POST /services/{id}/notifications
    async fn services_id_notifications_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdNotificationsPostPathParams,
        body: Multipart,
    ) -> Result<ServicesIdNotificationsPostResponse, ()>;
}
