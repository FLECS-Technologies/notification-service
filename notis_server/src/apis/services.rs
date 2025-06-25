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
pub enum ServicesGetResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdConfigPatchResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdDeleteResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdGetResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdPutResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

/// Services
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Services {
    /// Get a list of all registered services and their type.
    ///
    /// ServicesGet - GET /services
    async fn services_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<ServicesGetResponse, ()>;

    /// Manipulate the configuration of a notification service.
    ///
    /// ServicesIdConfigPatch - PATCH /services/{id}/config
    async fn services_id_config_patch(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdConfigPatchPathParams,
    ) -> Result<ServicesIdConfigPatchResponse, ()>;

    /// Delete the notification service.
    ///
    /// ServicesIdDelete - DELETE /services/{id}
    async fn services_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdDeletePathParams,
    ) -> Result<ServicesIdDeleteResponse, ()>;

    /// Get the notification service and their type.
    ///
    /// ServicesIdGet - GET /services/{id}
    async fn services_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdGetPathParams,
    ) -> Result<ServicesIdGetResponse, ()>;

    /// Create a new notification service, or replace an existing one.
    ///
    /// ServicesIdPut - PUT /services/{id}
    async fn services_id_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdPutPathParams,
    ) -> Result<ServicesIdPutResponse, ()>;
}
