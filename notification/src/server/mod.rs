mod api;

use axum::async_trait;
use axum::http::Method;
use notis_server::apis::notifications::{
    NotificationsPostResponse, ServicesIdNotificationsPostResponse,
};
use notis_server::apis::schema::{
    SchemaServicesServiceTypeConfigGetResponse, SchemaServicesServiceTypeNotificationGetResponse,
};
use notis_server::apis::services::{
    ServicesGetResponse, ServicesIdConfigPatchResponse, ServicesIdDeleteResponse,
    ServicesIdGetResponse, ServicesIdPutResponse,
};
use notis_server::models::{
    NotificationsPostRequest, SchemaServicesServiceTypeConfigGetPathParams,
    SchemaServicesServiceTypeNotificationGetPathParams, ServicesIdConfigPatchPathParams,
    ServicesIdDeletePathParams, ServicesIdGetPathParams, ServicesIdNotificationsPostPathParams,
    ServicesIdPutPathParams,
};

pub struct Server {}

#[async_trait]
impl notis_server::apis::services::Services for Server {
    async fn services_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
    ) -> Result<ServicesGetResponse, ()> {
        todo!()
    }

    async fn services_id_config_patch(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: ServicesIdConfigPatchPathParams,
    ) -> Result<ServicesIdConfigPatchResponse, ()> {
        todo!()
    }

    async fn services_id_delete(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: ServicesIdDeletePathParams,
    ) -> Result<ServicesIdDeleteResponse, ()> {
        todo!()
    }

    async fn services_id_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: ServicesIdGetPathParams,
    ) -> Result<ServicesIdGetResponse, ()> {
        todo!()
    }

    async fn services_id_put(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: ServicesIdPutPathParams,
    ) -> Result<ServicesIdPutResponse, ()> {
        todo!()
    }
}

#[async_trait]
impl notis_server::apis::notifications::Notifications for Server {
    async fn notifications_post(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        body: NotificationsPostRequest,
    ) -> Result<NotificationsPostResponse, ()> {
        Ok(api::notifications::post(body).await)
    }

    async fn services_id_notifications_post(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: ServicesIdNotificationsPostPathParams,
    ) -> Result<ServicesIdNotificationsPostResponse, ()> {
        todo!()
    }
}

#[async_trait]
impl notis_server::apis::schema::Schema for Server {
    async fn schema_services_service_type_config_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: SchemaServicesServiceTypeConfigGetPathParams,
    ) -> Result<SchemaServicesServiceTypeConfigGetResponse, ()> {
        todo!()
    }

    async fn schema_services_service_type_notification_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        _path_params: SchemaServicesServiceTypeNotificationGetPathParams,
    ) -> Result<SchemaServicesServiceTypeNotificationGetResponse, ()> {
        todo!()
    }
}
