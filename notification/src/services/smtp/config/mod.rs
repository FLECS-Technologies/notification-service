use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ConnectionType {
    Tls,
    StartTls,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server_url: String,
    pub credentials: lettre::transport::smtp::authentication::Credentials,
    pub connection_type: ConnectionType,
    pub sender: lettre::message::Mailbox,
    pub receivers: Vec<lettre::message::Mailbox>,
}

impl Config {
    pub fn example() -> Self {
        Self {
            server_url: "smtp.example.com".to_string(),
            credentials: lettre::transport::smtp::authentication::Credentials::new(
                "my_user".to_string(),
                "my_password".to_string(),
            ),
            connection_type: ConnectionType::Tls,
            sender: lettre::message::Mailbox::new(
                Some("Alice".to_string()),
                lettre::Address::new("alice", "mail.com").unwrap(),
            ),
            receivers: vec![
                lettre::message::Mailbox::new(
                    Some("Bob".to_string()),
                    lettre::Address::new("bob", "bob-self-hosting.com").unwrap(),
                ),
                lettre::message::Mailbox::new(
                    Some("Charlie".to_string()),
                    lettre::Address::new("charlie", "mail.ca").unwrap(),
                ),
            ],
        }
    }
}
