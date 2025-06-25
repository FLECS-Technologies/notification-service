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
pub enum SchemaServicesServiceTypeConfigGetResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SchemaServicesServiceTypeNotificationGetResponse {
    /// Not Implemented
    Status501_NotImplemented,
}

/// Schema
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Schema {
    /// Get the schema for patching the config for the given service type.
    ///
    /// SchemaServicesServiceTypeConfigGet - GET /schema/services/{service_type}/config
    async fn schema_services_service_type_config_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::SchemaServicesServiceTypeConfigGetPathParams,
    ) -> Result<SchemaServicesServiceTypeConfigGetResponse, ()>;

    /// Get the schema for optional additional data on notifications for the given service type.
    ///
    /// SchemaServicesServiceTypeNotificationGet - GET /schema/services/{service_type}/notification
    async fn schema_services_service_type_notification_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::SchemaServicesServiceTypeNotificationGetPathParams,
    ) -> Result<SchemaServicesServiceTypeNotificationGetResponse, ()>;
}
