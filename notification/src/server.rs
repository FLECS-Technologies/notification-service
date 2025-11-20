mod api;

use axum::async_trait;
use axum::http::Method;
use notis_server::apis::notifications::{
    NotificationsPostResponse, ServicesIdNotificationsPostResponse,
};
use notis_server::apis::services::{
    DefaultServiceDeleteResponse, DefaultServiceGetResponse, DefaultServicePostResponse,
    SchemaServiceTypesServiceTypeConfigGetResponse, ServicesGetResponse,
    ServicesIdConfigGetResponse, ServicesIdConfigPatchResponse, ServicesIdConfigSchemaGetResponse,
    ServicesIdDeleteResponse, ServicesIdGetResponse, ServicesIdNotificationsSchemaGetResponse,
    ServicesIdPutResponse,
};
use notis_server::models;
use notis_server::models::{
    DefaultServicePostRequest, NotificationsPostRequest,
    SchemaServiceTypesServiceTypeConfigGetPathParams, ServicesIdConfigGetPathParams,
    ServicesIdConfigPatchPathParams, ServicesIdConfigSchemaGetPathParams,
    ServicesIdConfigSchemaGetQueryParams, ServicesIdDeletePathParams, ServicesIdGetPathParams,
    ServicesIdNotificationsPostPathParams, ServicesIdNotificationsSchemaGetPathParams,
    ServicesIdPutPathParams, ServicesIdPutRequest,
};
use notis_server::types::Object;
use std::fmt::Display;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock, RwLockWriteGuard};

fn reason(reason: impl Display) -> models::Reason {
    models::Reason {
        reason: Some(reason.to_string()),
    }
}

pub struct Server {
    config: Arc<RwLock<crate::config::Config>>,
    config_path: PathBuf,
}

pub struct ConfigWriter<'a> {
    lock: RwLockWriteGuard<'a, crate::config::Config>,
    config_path: &'a Path,
    pub new_config: crate::config::Config,
}

impl ConfigWriter<'_> {
    pub fn write_config(mut self) -> std::io::Result<bool> {
        if self.new_config != *self.lock {
            serde_json::to_writer_pretty(
                std::fs::File::create(self.config_path)?,
                &self.new_config,
            )?;
            *self.lock = self.new_config.clone();
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Server {
    pub fn new(config: crate::config::Config, config_path: PathBuf) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            config_path,
        }
    }

    pub fn config_writer(&self) -> ConfigWriter {
        let lock = self.config.write().unwrap_or_else(|e| e.into_inner());
        ConfigWriter {
            new_config: lock.deref().clone(),
            lock,
            config_path: self.config_path.as_path(),
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
        let mut config_writer = self.config_writer();
        let result = api::default_service::delete(&mut config_writer.new_config);
        Ok(match config_writer.write_config() {
            Err(e) => DefaultServiceDeleteResponse::Status500_InternalServerError(reason(e)),
            Ok(_) => result,
        })
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
        let mut config_writer = self.config_writer();
        let result = api::default_service::post(&mut config_writer.new_config, body);
        Ok(match config_writer.write_config() {
            Err(e) => DefaultServicePostResponse::Status500_InternalServerError(reason(e)),
            Ok(_) => result,
        })
    }

    async fn schema_service_types_service_type_config_get(
        &self,
        _method: Method,
        _host: axum::extract::Host,
        _cookies: axum_extra::extract::cookie::CookieJar,
        path_params: SchemaServiceTypesServiceTypeConfigGetPathParams,
    ) -> Result<SchemaServiceTypesServiceTypeConfigGetResponse, ()> {
        Ok(api::schema::service_types::service_type::config::get(
            path_params,
        ))
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
        let mut config_writer = self.config_writer();
        let result =
            api::services::id::config::patch(&mut config_writer.new_config, path_params, body);
        Ok(match config_writer.write_config() {
            Err(e) => ServicesIdConfigPatchResponse::Status500_InternalServerError(reason(e)),
            Ok(_) => result,
        })
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
        let mut config_writer = self.config_writer();
        let result = api::services::id::delete(&mut config_writer.new_config, path_params);
        Ok(match config_writer.write_config() {
            Err(e) => ServicesIdDeleteResponse::Status500_InternalServerError(reason(e)),
            Ok(_) => result,
        })
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
        let mut config_writer = self.config_writer();
        let result = api::services::id::put(&mut config_writer.new_config, path_params, body);
        Ok(match config_writer.write_config() {
            Err(e) => ServicesIdPutResponse::Status500_InternalServerError(reason(e)),
            Ok(_) => result,
        })
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
        body: axum_extra::extract::Multipart,
    ) -> Result<ServicesIdNotificationsPostResponse, ()> {
        let request =
            match api::services::id::notifications::read_post_request_from_multipart(body).await {
                Ok(request) => request,
                Err(e) => return Ok(e),
            };
        let config = self.config.read().unwrap();
        Ok(
            api::services::id::notifications::post(&config, path_params, request)
                .unwrap_or_else(|e| e),
        )
    }
}
