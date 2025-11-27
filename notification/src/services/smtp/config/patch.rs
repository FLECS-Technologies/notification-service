use crate::services::smtp::{ConnectionType, Mailbox};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Deserialize, Serialize, JsonSchema)]
pub struct ConfigPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "super::schema::Credentials")]
    pub credentials: Option<lettre::transport::smtp::authentication::Credentials>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<ConnectionType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schemars(with = "Option<Option<super::schema::Mechanism>>")]
    pub auth_mechanism: Option<Option<lettre::transport::smtp::authentication::Mechanism>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Mailbox>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<Mailbox>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_groups: Option<HashMap<String, Vec<Mailbox>>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    #[schemars(with = "Option<Option<usize>>")]
    pub total_attachment_size_limit: Option<Option<usize>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use lettre::transport::smtp::authentication::Mechanism;

    #[test]
    fn te() {
        let s = r#"{}"#;
        let p = ConfigPatch {
            server_url: None,
            credentials: None,
            connection_type: None,
            auth_mechanism: None,
            sender: None,
            receivers: None,
            receiver_groups: None,
            total_attachment_size_limit: None,
        };
        assert_eq!(p, serde_json::from_str(s).unwrap());
        assert_eq!(s, serde_json::to_string(&p).unwrap());

        // Unset Value
        let s =
            r#"{"auth_mechanism":null,"receiver_groups":{},"total_attachment_size_limit":null}"#;
        let p = ConfigPatch {
            server_url: None,
            credentials: None,
            connection_type: None,
            auth_mechanism: Some(None),
            sender: None,
            receivers: None,
            receiver_groups: Some(HashMap::new()),
            total_attachment_size_limit: Some(None),
        };
        assert_eq!(p, serde_json::from_str(s).unwrap());
        assert_eq!(s, serde_json::to_string(&p).unwrap());

        // Existing Value
        let s = r#"{"auth_mechanism":"Login","receiver_groups":{"gods":[{"name":"Zeus","email":"godfather@olympus.gr"},{"name":"Hera","email":"moon@olympus.gr"}]},"total_attachment_size_limit":100}"#;
        let p = ConfigPatch {
            server_url: None,
            credentials: None,
            connection_type: None,
            auth_mechanism: Some(Some(Mechanism::Login)),
            sender: None,
            receivers: None,
            receiver_groups: Some(HashMap::from([(
                "gods".to_string(),
                vec![
                    Mailbox {
                        name: Some("Zeus".to_string()),
                        email: lettre::Address::new("godfather", "olympus.gr").unwrap(),
                    },
                    Mailbox {
                        name: Some("Hera".to_string()),
                        email: lettre::Address::new("moon", "olympus.gr").unwrap(),
                    },
                ],
            )])),
            total_attachment_size_limit: Some(Some(100)),
        };
        assert_eq!(p, serde_json::from_str(s).unwrap());
        assert_eq!(s, serde_json::to_string(&p).unwrap());
    }
}
