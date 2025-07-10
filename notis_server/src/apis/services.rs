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
pub enum DefaultServiceDeleteResponse {
    /// Success
    Status200_Success,
    /// No default service registered
    Status404_NoDefaultServiceRegistered,
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DefaultServiceGetResponse {
    /// Success
    Status200_Success(models::DefaultServiceGet200Response),
    /// No default service registered
    Status404_NoDefaultServiceRegistered,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DefaultServicePostResponse {
    /// Default service was replaced
    Status200_DefaultServiceWasReplaced,
    /// Default service was set
    Status201_DefaultServiceWasSet,
    /// Service not found
    Status404_ServiceNotFound,
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SchemaServiceTypesServiceTypeConfigGetResponse {
    /// Success
    Status200_Success(crate::types::Object),
    /// Service type not found
    Status404_ServiceTypeNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesGetResponse {
    /// Success
    Status200_Success(std::collections::HashMap<String, String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdConfigGetResponse {
    /// Success
    Status200_Success(crate::types::Object),
    /// Service not found
    Status404_ServiceNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdConfigPatchResponse {
    /// Success
    Status200_Success,
    /// Bad Request
    Status400_BadRequest(models::Reason),
    /// Service not found
    Status404_ServiceNotFound,
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdConfigSchemaGetResponse {
    /// Success
    Status200_Success(crate::types::Object),
    /// Service not found
    Status404_ServiceNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdDeleteResponse {
    /// Success
    Status200_Success,
    /// Service not found
    Status404_ServiceNotFound,
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdGetResponse {
    /// Success
    Status200_Success(models::ServicesIdGet200Response),
    /// Service not found
    Status404_ServiceNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdNotificationsSchemaGetResponse {
    /// Success
    Status200_Success(crate::types::Object),
    /// Service not found
    Status404_ServiceNotFound,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ServicesIdPutResponse {
    /// Service was replaced
    Status200_ServiceWasReplaced,
    /// Service was created
    Status201_ServiceWasCreated,
    /// Bad Request
    Status400_BadRequest(models::Reason),
    /// Internal Server Error
    Status500_InternalServerError(models::Reason),
}

/// Services
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Services {
    /// Remove the reference to the default service.
    ///
    /// DefaultServiceDelete - DELETE /default_service
    async fn default_service_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<DefaultServiceDeleteResponse, ()>;

    /// Get the default service and its type.
    ///
    /// DefaultServiceGet - GET /default_service
    async fn default_service_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<DefaultServiceGetResponse, ()>;

    /// Set the default service.
    ///
    /// DefaultServicePost - POST /default_service
    async fn default_service_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::DefaultServicePostRequest,
    ) -> Result<DefaultServicePostResponse, ()>;

    /// Get the configuration schema of a service type.
    ///
    /// SchemaServiceTypesServiceTypeConfigGet - GET /schema/service_types/{service_type}/config
    async fn schema_service_types_service_type_config_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::SchemaServiceTypesServiceTypeConfigGetPathParams,
    ) -> Result<SchemaServiceTypesServiceTypeConfigGetResponse, ()>;

    /// Get a list of all registered services, their type and the default service.
    ///
    /// ServicesGet - GET /services
    async fn services_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<ServicesGetResponse, ()>;

    /// Get the configuration of a notification service.
    ///
    /// ServicesIdConfigGet - GET /services/{id}/config
    async fn services_id_config_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdConfigGetPathParams,
    ) -> Result<ServicesIdConfigGetResponse, ()>;

    /// Manipulate the configuration of a notification service.
    ///
    /// ServicesIdConfigPatch - PATCH /services/{id}/config
    async fn services_id_config_patch(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdConfigPatchPathParams,
        body: crate::types::Object,
    ) -> Result<ServicesIdConfigPatchResponse, ()>;

    /// Get the configuration schema of a notification service.
    ///
    /// ServicesIdConfigSchemaGet - GET /services/{id}/config/schema
    async fn services_id_config_schema_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdConfigSchemaGetPathParams,
        query_params: models::ServicesIdConfigSchemaGetQueryParams,
    ) -> Result<ServicesIdConfigSchemaGetResponse, ()>;

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

    /// Get the schema for sending notifications via the notification service.
    ///
    /// ServicesIdNotificationsSchemaGet - GET /services/{id}/notifications/schema
    async fn services_id_notifications_schema_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdNotificationsSchemaGetPathParams,
    ) -> Result<ServicesIdNotificationsSchemaGetResponse, ()>;

    /// Create a new notification service, or replace an existing one.
    ///
    /// ServicesIdPut - PUT /services/{id}
    async fn services_id_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ServicesIdPutPathParams,
        body: models::ServicesIdPutRequest,
    ) -> Result<ServicesIdPutResponse, ()>;
}
