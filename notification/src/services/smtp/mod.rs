mod config;
pub use config::*;
use lettre::transport::smtp::authentication::Mechanism;
use std::time::Duration;
use tracing::{error, info, info_span};

pub struct MailServer<'a> {
    config: &'a Config,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Address(#[from] lettre::address::AddressError),
    #[error(transparent)]
    Mail(#[from] lettre::error::Error),
    #[error(transparent)]
    Smtp(#[from] lettre::transport::smtp::Error),
    #[error("Failed to create encrypted tls connection")]
    Tls,
}

fn supports_feature(feature: &str, response: &lettre::transport::smtp::response::Response) -> bool {
    response
        .message()
        .skip(1)
        .any(|line| line.split_whitespace().next() == Some(feature))
}

impl<'a> MailServer<'a> {
    pub fn new_from_config(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn send_mail(&self, subject: &str, content: Option<String>) -> Result<(), Error> {
        let _span = info_span!(
            "send_mail",
            sender = <lettre::Address as AsRef<str>>::as_ref(&self.config.sender.email),
            server = self.config.server_url,
        )
        .entered();
        let mut mail_builder = lettre::Message::builder()
            .from(self.config.sender.clone())
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_PLAIN);
        for mailbox in &self.config.receivers {
            mail_builder = mail_builder.to(mailbox.clone())
        }
        let email = mail_builder.body(content.unwrap_or_default())?;

        let tls = lettre::transport::smtp::client::TlsParameters::new(
            self.config.server_url.as_str().into(),
        )?;
        let client_id = lettre::transport::smtp::extension::ClientId::default();
        let mut connection = match self.config.connection_type {
            ConnectionType::StartTls => {
                let mut connection = lettre::transport::smtp::client::SmtpConnection::connect(
                    (
                        self.config.server_url.as_str(),
                        lettre::transport::smtp::SUBMISSION_PORT,
                    ),
                    Some(Duration::from_secs(10)),
                    &client_id,
                    None,
                    None,
                )?;
                connection.starttls(&tls, &client_id)?;
                connection
            }
            ConnectionType::Tls => lettre::transport::smtp::client::SmtpConnection::connect(
                (
                    self.config.server_url.as_str(),
                    lettre::transport::smtp::SUBMISSIONS_PORT,
                ),
                Some(Duration::from_secs(10)),
                &lettre::transport::smtp::extension::ClientId::default(),
                Some(&tls),
                None,
            )?,
        };
        if !connection.is_encrypted() {
            return Err(Error::Tls);
        }
        connection.auth(&[Mechanism::Login], &self.config.credentials)?;
        let ehlo = connection.command(lettre::transport::smtp::commands::Ehlo::new(client_id))?;
        let supports_sdn = supports_feature("DSN", &ehlo);
        let mail_parameters = if supports_sdn {
            vec![lettre::transport::smtp::extension::MailParameter::Other {
                keyword: "RET".to_string(),
                value: Some("FULL".to_string()),
            }]
        } else {
            Vec::new()
        };
        let mail = lettre::transport::smtp::commands::Mail::new(
            email.envelope().from().cloned(),
            mail_parameters,
        );
        connection.command(mail)?;
        let rcpt_parameters = if supports_sdn {
            vec![lettre::transport::smtp::extension::RcptParameter::Other {
                keyword: "NOTIFY".to_string(),
                value: Some("FAILURE,DELAY".to_string()),
            }]
        } else {
            Vec::new()
        };
        for to in email.envelope().to() {
            let rcpt =
                lettre::transport::smtp::commands::Rcpt::new(to.clone(), rcpt_parameters.clone());
            connection.command(rcpt)?;
        }
        connection.command(lettre::transport::smtp::commands::Data)?;
        let data = email.formatted();
        info!("Sending email...");
        if let Err(e) = connection.message(&data) {
            error!("{e}");
            Err(e.into())
        } else {
            info!("... Ok");
            Ok(())
        }
    }
}
