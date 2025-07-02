mod api;

use axum::async_trait;
use axum::http::Method;
use notis_server::apis::notifications::{
    NotificationsPostResponse, ServicesIdNotificationsPostResponse,
};
use notis_server::apis::services::{
    DefaultServiceDeleteResponse, DefaultServiceGetResponse, DefaultServicePostResponse,
    ServicesGetResponse, ServicesIdConfigGetResponse, ServicesIdConfigPatchResponse,
    ServicesIdConfigSchemaGetResponse, ServicesIdDeleteResponse, ServicesIdGetResponse,
    ServicesIdNotificationsSchemaGetResponse, ServicesIdPutResponse,
};
use notis_server::models::{
    DefaultServicePostRequest, NotificationsPostRequest, ServicesIdConfigGetPathParams,
    ServicesIdConfigPatchPathParams, ServicesIdConfigSchemaGetPathParams,
    ServicesIdConfigSchemaGetQueryParams, ServicesIdDeletePathParams, ServicesIdGetPathParams,
    ServicesIdNotificationsPostPathParams, ServicesIdNotificationsPostRequest,
    ServicesIdNotificationsSchemaGetPathParams, ServicesIdPutPathParams, ServicesIdPutRequest,
};
use notis_server::types::Object;
use std::sync::{Arc, RwLock};

pub struct Server {
    config: Arc<RwLock<crate::config::Config>>,
}

impl Server {
    pub fn new(config: crate::config::Config) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
        }
    }
}

#[async_trait]
impl notis_server::apis::services::Services for Server {
    async fn default_service_delete(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
    ) -> Result<DefaultServiceDeleteResponse, ()> {
        let mut config = self.config.write().unwrap();
        Ok(api::default_service::delete(&mut config))
    }

    async fn default_service_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
    ) -> Result<DefaultServiceGetResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::default_service::get(&config))
    }

    async fn default_service_post(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        body: DefaultServicePostRequest,
    ) -> Result<DefaultServicePostResponse, ()> {
        let mut config = self.config.write().unwrap();
        Ok(api::default_service::post(&mut config, body))
    }

    async fn services_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
    ) -> Result<ServicesGetResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::services::get(&config))
    }

    async fn services_id_config_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdConfigGetPathParams,
    ) -> Result<ServicesIdConfigGetResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::services::id::config::get(&config, path_params))
    }

    async fn services_id_config_patch(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdConfigPatchPathParams,
        body: Object,
    ) -> Result<ServicesIdConfigPatchResponse, ()> {
        let mut config = self.config.write().unwrap();
        Ok(api::services::id::config::patch(
            &mut config,
            path_params,
            body,
        ))
    }

    async fn services_id_config_schema_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdConfigSchemaGetPathParams,
        query_params: ServicesIdConfigSchemaGetQueryParams,
    ) -> Result<ServicesIdConfigSchemaGetResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::services::id::config::schema::get(
            &config,
            path_params,
            query_params,
        ))
    }

    async fn services_id_delete(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdDeletePathParams,
    ) -> Result<ServicesIdDeleteResponse, ()> {
        let mut config = self.config.write().unwrap();
        Ok(api::services::id::delete(&mut config, path_params))
    }

    async fn services_id_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdGetPathParams,
    ) -> Result<ServicesIdGetResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::services::id::get(&config, path_params))
    }

    async fn services_id_notifications_schema_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdNotificationsSchemaGetPathParams,
    ) -> Result<ServicesIdNotificationsSchemaGetResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::services::id::notifications::schema::get(
            &config,
            path_params,
        ))
    }

    async fn services_id_put(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdPutPathParams,
        body: ServicesIdPutRequest,
    ) -> Result<ServicesIdPutResponse, ()> {
        let mut config = self.config.write().unwrap();
        Ok(api::services::id::put(&mut config, path_params, body))
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
        let config = self.config.read().unwrap();
        Ok(api::notifications::post(&config, body))
    }

    async fn services_id_notifications_post(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: ServicesIdNotificationsPostPathParams,
        body: ServicesIdNotificationsPostRequest,
    ) -> Result<ServicesIdNotificationsPostResponse, ()> {
        let config = self.config.read().unwrap();
        Ok(api::services::id::notifications::post(
            &config,
            path_params,
            body,
        ))
    }
}
