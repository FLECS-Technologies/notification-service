mod config;
pub use config::*;
use lettre::Transport;
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
}

impl<'a> MailServer<'a> {
    pub fn new_from_config(config: &'a Config) -> Self {
        Self { config }
    }

    pub fn send_mail(&self, subject: &str, content: Option<String>) -> Result<(), Error> {
        let _span = info_span!(
            "send_mail",
            sender = <lettre::Address as AsRef<str>>::as_ref(&self.config.default_sender.email),
            receiver = <lettre::Address as AsRef<str>>::as_ref(&self.config.default_receiver.email),
            server = self.config.server_url,
        )
        .entered();
        info!("Creating email...");
        let email = lettre::Message::builder()
            .from(self.config.default_sender.clone())
            .to(self.config.default_receiver.clone())
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_PLAIN)
            .body(content.unwrap_or_default())?;
        info!("... Ok");

        info!("Connecting to smpt server...");
        let mailer = match self.config.connection_type {
            ConnectionType::StartTls => {
                lettre::SmtpTransport::starttls_relay(&self.config.server_url)?
                    .credentials(self.config.credentials.clone())
                    .build()
            }
            ConnectionType::Tls => lettre::SmtpTransport::relay(&self.config.server_url)?
                .credentials(self.config.credentials.clone())
                .build(),
        };
        info!("... Ok");

        info!("Sending email...");
        if let Err(e) = mailer.send(&email) {
            error!("{e}");
            Err(e.into())
        } else {
            info!("... Ok");
            Ok(())
        }
    }
}
