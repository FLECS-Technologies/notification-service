mod config;

use crate::services::{Attachment, NotificationService};
pub use config::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};
use std::time::Duration;
use tracing::{error, info, info_span};
use zip::write::FileOptions;
use zip::{AesMode, CompressionMethod, ZipWriter};

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
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Failed to create encrypted tls connection")]
    Tls,
    #[error(
        "The total size limit of attachments ({limit}bytes) was exceeded (total size = {total}bytes"
    )]
    TotalAttachmentSizeLimitExceeded { limit: usize, total: usize },
    #[error("The receiver group {group} is not configured")]
    UnknownReceiverGroup { group: String },
}

impl From<Attachment> for lettre::message::SinglePart {
    fn from(value: Attachment) -> Self {
        lettre::message::Attachment::new(value.file_name)
            .body(value.file_content, value.content_type)
    }
}

fn supports_feature(feature: &str, response: &lettre::transport::smtp::response::Response) -> bool {
    response
        .message()
        .skip(1)
        .any(|line| line.split_whitespace().next() == Some(feature))
}

#[derive(JsonSchema, Deserialize, Serialize)]
pub struct NotificationOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    receivers: Option<Vec<Mailbox>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    receiver_groups: Vec<String>,
}

impl NotificationOptions {
    fn create_receiver_list(&self, config: &Config) -> Result<Vec<Mailbox>, Error> {
        let mut receivers = match &self.receivers {
            None if self.receiver_groups.is_empty() => return Ok(config.receivers.clone()),
            Some(receivers) => receivers.clone(),
            _ => Vec::new(),
        };
        for group in &self.receiver_groups {
            receivers.extend_from_slice(config.receiver_groups.get(group).ok_or_else(|| {
                Error::UnknownReceiverGroup {
                    group: group.clone(),
                }
            })?)
        }
        Ok(receivers)
    }
}

impl NotificationService for MailServer {
    type Config = config::Config;
    type NotificationOptions = NotificationOptions;

    fn send_notification(
        &self,
        options: Option<Self::NotificationOptions>,
        config: &Self::Config,
        title: &str,
        attachments: Vec<Attachment>,
        content: Option<&str>,
    ) -> Result<(), crate::Error> {
        self.send_mail(
            config,
            title,
            content.map(str::to_string),
            attachments,
            options
                .map(|options| options.create_receiver_list(config))
                .transpose()?
                .unwrap_or_else(|| config.receivers.clone()),
        )?;
        Ok(())
    }
}

impl MailServer {
    fn prepare_attachments(
        attachments: Vec<Attachment>,
        encryption_password: Option<&str>,
    ) -> Result<Vec<Attachment>, Error> {
        if !attachments.is_empty()
            && let Some(password) = encryption_password
        {
            let buffer = Cursor::new(Vec::<u8>::new());
            let mut zip = ZipWriter::new(buffer);

            let options = FileOptions::<()>::default()
                .compression_method(CompressionMethod::Deflated)
                .with_aes_encryption(AesMode::Aes256, password);

            for Attachment {
                file_name,
                file_content,
                ..
            } in attachments
            {
                let name = file_name.replace('\\', "/");
                zip.start_file(name, options)?;
                zip.write_all(&file_content)?;
            }

            let cursor = zip.finish()?;
            Ok(vec![Attachment {
                file_name: "attachments.zip".to_string(),
                content_type: "application/zip".parse().unwrap(),
                file_content: cursor.into_inner(),
            }])
        } else {
            Ok(attachments)
        }
    }

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
        attachments: Vec<Attachment>,
        receivers: Vec<Mailbox>,
    ) -> Result<(), Error> {
        let _span = info_span!(
            "send_mail",
            sender = <lettre::Address as AsRef<str>>::as_ref(&config.sender.email),
            server = config.server_url,
        )
        .entered();
        let attachments =
            Self::prepare_attachments(attachments, config.encryption_password.as_deref())?;
        if let Some(total_attachment_size_limit) = config.total_attachment_size_limit {
            let total_attachment_size: usize = attachments
                .iter()
                .map(|attachment| attachment.file_content.len())
                .sum();
            if total_attachment_size > total_attachment_size_limit {
                return Err(Error::TotalAttachmentSizeLimitExceeded {
                    total: total_attachment_size,
                    limit: total_attachment_size_limit,
                });
            }
        }
        let mut mail_builder = lettre::Message::builder()
            .from(config.sender.clone().into())
            .subject(subject);
        for mailbox in receivers {
            mail_builder = mail_builder.to(mailbox.into())
        }
        let mut multipart = lettre::message::MultiPart::mixed().singlepart(
            lettre::message::SinglePart::plain(content.unwrap_or_default()),
        );
        for attachment in attachments {
            multipart = multipart.singlepart(attachment.into())
        }
        let email = mail_builder.multipart(multipart)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_attachment(name: &str, content: &[u8]) -> Attachment {
        Attachment {
            file_name: name.to_string(),
            content_type: "application/octet-stream".parse().unwrap(),
            file_content: content.to_vec(),
        }
    }

    // No encryption, no attachments → empty Vec, no ZIP produced
    #[test]
    fn no_encryption_no_attachments() {
        let result = MailServer::prepare_attachments(vec![], None).unwrap();
        assert!(result.is_empty());
    }

    // No encryption, some attachments → original attachments passed through unchanged
    #[test]
    fn no_encryption_with_attachments() {
        let attachments = vec![
            make_attachment("a.txt", b"hello"),
            make_attachment("b.txt", b"world"),
        ];
        let result = MailServer::prepare_attachments(attachments, None).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].file_name, "a.txt");
        assert_eq!(result[0].file_content, b"hello");
        assert_eq!(result[1].file_name, "b.txt");
        assert_eq!(result[1].file_content, b"world");
    }

    // Encryption set, no attachments → empty Vec, no ZIP produced
    #[test]
    fn with_encryption_no_attachments() {
        let result = MailServer::prepare_attachments(vec![], Some("secret")).unwrap();
        assert!(
            result.is_empty(),
            "expected no output attachments when input is empty, even with encryption configured"
        );
    }

    // Encryption set, some attachments → single ZIP attachment containing all files
    #[test]
    fn with_encryption_with_attachments() {
        let attachments = vec![
            make_attachment("report.txt", b"report content"),
            make_attachment("data.csv", b"col1,col2\n1,2"),
        ];
        let result = MailServer::prepare_attachments(attachments, Some("secret")).unwrap();

        assert_eq!(result.len(), 1);
        let zip_attachment = &result[0];
        assert_eq!(zip_attachment.file_name, "attachments.zip");
        assert_eq!(
            zip_attachment.content_type,
            "application/zip".parse().unwrap()
        );

        // Verify the ZIP is valid and contains the expected files
        let cursor = Cursor::new(zip_attachment.file_content.clone());
        let mut archive = zip::ZipArchive::new(cursor).expect("output should be a valid ZIP");
        assert_eq!(archive.len(), 2);

        let mut names: Vec<String> = (0..archive.len())
            .map(|i| archive.by_index_raw(i).unwrap().name().to_string())
            .collect();
        names.sort();
        assert_eq!(names, ["data.csv", "report.txt"]);
    }
}
