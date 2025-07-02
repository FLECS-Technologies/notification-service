use crate::services::smtp::{ConnectionType, Mailbox};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
        };
        assert_eq!(p, serde_json::from_str(s).unwrap());
        assert_eq!(s, serde_json::to_string(&p).unwrap());

        // Unset Value
        let s = r#"{"auth_mechanism":null}"#;
        let p = ConfigPatch {
            server_url: None,
            credentials: None,
            connection_type: None,
            auth_mechanism: Some(None),
            sender: None,
            receivers: None,
        };
        assert_eq!(p, serde_json::from_str(s).unwrap());
        assert_eq!(s, serde_json::to_string(&p).unwrap());

        // Existing Value
        let s = r#"{"auth_mechanism":"Login"}"#;
        let p = ConfigPatch {
            server_url: None,
            credentials: None,
            connection_type: None,
            auth_mechanism: Some(Some(Mechanism::Login)),
            sender: None,
            receivers: None,
        };
        assert_eq!(p, serde_json::from_str(s).unwrap());
        assert_eq!(s, serde_json::to_string(&p).unwrap());
    }
}
