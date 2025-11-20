pub mod schema;

use crate::config::NotificationServiceConfig;
use crate::services::NotisNotificationService;
use notis_server::apis::services::ServicesIdConfigGetResponse as GetResponse;
use notis_server::apis::services::ServicesIdConfigPatchResponse as PatchResponse;
use notis_server::models::ServicesIdConfigGetPathParams as GetPathParams;
use notis_server::models::ServicesIdConfigPatchPathParams as PatchPathParams;
use notis_server::types;

type PatchRequest = types::Object;

pub fn get(config: &crate::config::Config, path_params: GetPathParams) -> GetResponse {
    match &config.notification_services.get(&path_params.id) {
        Some(NotisNotificationService::LOG(config)) => {
            GetResponse::Status200_Success(types::Object(serde_json::to_value(config).unwrap()))
        }
        &Some(NotisNotificationService::SMTP(config)) => GetResponse::Status200_Success(
            types::Object(serde_json::to_value(&config.redacted()).unwrap()),
        ),
        None => GetResponse::Status404_ServiceNotFound,
    }
}

pub fn patch(
    config: &mut crate::config::Config,
    path_params: PatchPathParams,
    request: PatchRequest,
) -> PatchResponse {
    match config.notification_services.get_mut(&path_params.id) {
        Some(NotisNotificationService::LOG(config)) => {
            let patch: crate::services::log::Config = serde_json::from_value(request.0).unwrap();
            config.apply_patch(patch);
            PatchResponse::Status200_Success
        }
        Some(NotisNotificationService::SMTP(config)) => {
            let patch: crate::services::smtp::ConfigPatch =
                serde_json::from_value(request.0).unwrap();
            config.apply_patch(patch);
            PatchResponse::Status200_Success
        }
        None => PatchResponse::Status404_ServiceNotFound,
    }
}
