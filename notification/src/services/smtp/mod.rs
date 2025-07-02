mod config;
use crate::services::NotificationService;
pub use config::*;
use std::time::Duration;
use tracing::{error, info, info_span};

#[derive(Default)]
pub struct MailServer;

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

impl NotificationService for MailServer {
    type Config = config::Config;
    type NotificationOptions = Vec<Mailbox>;

    fn send_notification(
        &self,
        options: Option<Self::NotificationOptions>,
        config: &Self::Config,
        title: &str,
        content: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.send_mail(
            config,
            title,
            content.map(str::to_string),
            options.unwrap_or_else(|| config.receivers.clone()),
        )?;
        Ok(())
    }
}

impl MailServer {
    fn tls_connect(
        client_id: &lettre::transport::smtp::extension::ClientId,
        tls: &lettre::transport::smtp::client::TlsParameters,
        server_url: &str,
    ) -> Result<lettre::transport::smtp::client::SmtpConnection, Error> {
        let connection = lettre::transport::smtp::client::SmtpConnection::connect(
            (server_url, lettre::transport::smtp::SUBMISSIONS_PORT),
            Some(Duration::from_secs(10)),
            client_id,
            Some(tls),
            None,
        )?;
        if !connection.is_encrypted() {
            return Err(Error::Tls);
        }
        Ok(connection)
    }

    fn start_tls_connect(
        client_id: &lettre::transport::smtp::extension::ClientId,
        tls: &lettre::transport::smtp::client::TlsParameters,
        server_url: &str,
    ) -> Result<lettre::transport::smtp::client::SmtpConnection, Error> {
        let mut connection = lettre::transport::smtp::client::SmtpConnection::connect(
            (server_url, lettre::transport::smtp::SUBMISSION_PORT),
            Some(Duration::from_secs(10)),
            client_id,
            None,
            None,
        )?;
        connection.starttls(tls, client_id)?;
        if !connection.is_encrypted() {
            return Err(Error::Tls);
        }
        Ok(connection)
    }

    fn plain_unsecure_connect(
        client_id: &lettre::transport::smtp::extension::ClientId,
        server_url: &str,
    ) -> Result<lettre::transport::smtp::client::SmtpConnection, Error> {
        let connection = lettre::transport::smtp::client::SmtpConnection::connect(
            (server_url, lettre::transport::smtp::SMTP_PORT),
            Some(Duration::from_secs(10)),
            client_id,
            None,
            None,
        )?;
        Ok(connection)
    }

    pub fn send_mail(
        &self,
        config: &Config,
        subject: &str,
        content: Option<String>,
        receivers: Vec<Mailbox>,
    ) -> Result<(), Error> {
        let _span = info_span!(
            "send_mail",
            sender = <lettre::Address as AsRef<str>>::as_ref(&config.sender.email),
            server = config.server_url,
        )
        .entered();
        let mut mail_builder = lettre::Message::builder()
            .from(config.sender.clone().into())
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_PLAIN);
        for mailbox in receivers {
            mail_builder = mail_builder.to(mailbox.into())
        }
        let email = mail_builder.body(content.unwrap_or_default())?;

        let tls =
            lettre::transport::smtp::client::TlsParameters::new(config.server_url.as_str().into())?;
        let client_id = lettre::transport::smtp::extension::ClientId::default();
        let mut connection = match config.connection_type {
            ConnectionType::StartTls => {
                Self::start_tls_connect(&client_id, &tls, config.server_url.as_str())?
            }
            ConnectionType::Tls => Self::tls_connect(&client_id, &tls, config.server_url.as_str())?,
            ConnectionType::PlainUnsecure => {
                Self::plain_unsecure_connect(&client_id, config.server_url.as_str())?
            }
        };
        if let Some(mechanism) = config.auth_mechanism {
            connection.auth(&[mechanism], &config.credentials)?;
        }
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
                value: Some("FAILURE,DELAY,SUCCESS".to_string()),
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
