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
pub enum AppsInstallPostResponse {
    /// Accepted
    Status202_Accepted,
    /// Bad Request
    Status400_BadRequest,
    /// Internal Server Error
    Status500_InternalServerError,
}

/// Apps
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Apps {
    /// Install an App from the FLECS marketplace.
    ///
    /// AppsInstallPost - POST /apps/install
    async fn apps_install_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::AppsInstallPostRequest,
    ) -> Result<AppsInstallPostResponse, ()>;
}
