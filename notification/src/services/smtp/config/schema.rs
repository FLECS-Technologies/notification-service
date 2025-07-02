use schemars::JsonSchema;

#[derive(JsonSchema)]
#[serde(remote = "lettre::transport::smtp::authentication::Credentials")]
#[allow(dead_code)]
pub(super) struct Credentials {
    authentication_identity: String,
    secret: String,
}

#[derive(JsonSchema)]
#[serde(remote = "lettre::transport::smtp::authentication::Mechanism")]
#[allow(dead_code)]
pub(super) enum Mechanism {
    Plain,
    Login,
    Xoauth2,
}
