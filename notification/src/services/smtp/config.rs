mod patch;
mod schema;

use crate::config::NotificationServiceConfig;
pub use patch::ConfigPatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
pub enum ConnectionType {
    Tls,
    StartTls,
    PlainUnsecure,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    pub server_url: String,
    #[schemars(with = "schema::Credentials")]
    pub credentials: lettre::transport::smtp::authentication::Credentials,
    pub connection_type: ConnectionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Option<schema::Mechanism>")]
    pub auth_mechanism: Option<lettre::transport::smtp::authentication::Mechanism>,
    pub sender: Mailbox,
    pub receivers: Vec<Mailbox>,
    pub total_attachment_size_limit: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct Mailbox {
    pub name: Option<String>,
    #[schemars(email, with = "String")]
    pub email: lettre::Address,
}

impl From<Mailbox> for lettre::message::Mailbox {
    fn from(value: Mailbox) -> Self {
        lettre::message::Mailbox::new(value.name, value.email)
    }
}

impl Config {
    pub fn example() -> Self {
        Self {
            server_url: "smtp.example.com".to_string(),
            credentials: lettre::transport::smtp::authentication::Credentials::new(
                "my_user".to_string(),
                "my_password".to_string(),
            ),
            auth_mechanism: Some(lettre::transport::smtp::authentication::Mechanism::Login),
            connection_type: ConnectionType::Tls,
            sender: Mailbox {
                name: Some("Alice".to_string()),
                email: lettre::Address::new("alice", "mail.com").unwrap(),
            },
            receivers: vec![
                Mailbox {
                    name: Some("Bob".to_string()),
                    email: lettre::Address::new("bob", "bob-self-hosting.com").unwrap(),
                },
                Mailbox {
                    name: Some("Charlie".to_string()),
                    email: lettre::Address::new("charlie", "mail.ca").unwrap(),
                },
            ],
            total_attachment_size_limit: Some(1024 * 1024 * 100),
        }
    }

    pub fn redacted(&self) -> Self {
        let credentials_json: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&self.credentials).unwrap()).unwrap();
        let username = credentials_json
            .as_object()
            .unwrap()
            .get("authentication_identity")
            .unwrap()
            .to_string();
        Self {
            credentials: lettre::transport::smtp::authentication::Credentials::new(
                username,
                "***".to_string(),
            ),
            ..self.clone()
        }
    }
}

impl NotificationServiceConfig for Config {
    type Patch = ConfigPatch;

    fn apply_patch(&mut self, patch: ConfigPatch) {
        if let Some(server_url) = patch.server_url {
            self.server_url = server_url;
        }
        if let Some(credentials) = patch.credentials {
            self.credentials = credentials;
        }
        if let Some(auth_mechanism) = patch.auth_mechanism {
            self.auth_mechanism = auth_mechanism;
        }
        if let Some(connection_type) = patch.connection_type {
            self.connection_type = connection_type;
        }
        if let Some(sender) = patch.sender {
            self.sender = sender;
        }
        if let Some(receivers) = patch.receivers {
            self.receivers = receivers;
        }
    }
}
