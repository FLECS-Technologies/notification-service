use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ConnectionType {
    Tls,
    StartTls,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub id: String,
    pub server_url: String,
    pub credentials: lettre::transport::smtp::authentication::Credentials,
    pub connection_type: ConnectionType,
    pub default_sender: lettre::message::Mailbox,
    pub default_receiver: lettre::message::Mailbox,
}

impl Config {
    pub fn example() -> Self {
        Self {
            id: "example-smtp-config".to_string(),
            server_url: "smtp.example.com".to_string(),
            credentials: lettre::transport::smtp::authentication::Credentials::new(
                "my_user".to_string(),
                "my_password".to_string(),
            ),
            connection_type: ConnectionType::Tls,
            default_sender: lettre::message::Mailbox::new(
                Some("Alice".to_string()),
                lettre::Address::new("alice", "mail.com").unwrap(),
            ),
            default_receiver: lettre::message::Mailbox::new(
                Some("Alice".to_string()),
                lettre::Address::new("bob", "bob-self-hosting.com").unwrap(),
            ),
        }
    }
}
