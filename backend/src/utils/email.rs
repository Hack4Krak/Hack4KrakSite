use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use askama::DynTemplate;
use lettre::message::header::{HeaderName, HeaderValue};
use lettre::message::{Attachment, Mailbox, MultiPart, SinglePart, header};
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::str::FromStr;
use utoipa::ToSchema;

pub const UNDISCLOSED_RECIPIENTS: &str = "undisclosed-recipients:;";
pub const TO_EMAIL_HEADER: HeaderName = HeaderName::new_from_ascii_str("To");

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct EmailMeta {
    pub subject: String,
    pub sender_name: Option<String>,
}

impl Default for EmailMeta {
    fn default() -> Self {
        EmailMeta {
            subject: "Hack4Krak".to_string(),
            sender_name: Some("Kontakt Hack4Krak".to_string()),
        }
    }
}

impl<T: DynTemplate> EmailTemplate for T {}

pub trait EmailTemplate: DynTemplate {
    fn id(&self) -> &str {
        type_name::<Self>().rsplit("::").next().unwrap()
    }
}

pub trait SmtpClient: Send + Sync {
    fn send(&self, email: &Message) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub content_type: header::ContentType,
    pub body: Vec<u8>,
}

impl EmailAttachment {
    pub fn new(
        filename: impl Into<String>,
        content_type: header::ContentType,
        body: Vec<u8>,
    ) -> Self {
        Self {
            filename: filename.into(),
            content_type,
            body,
        }
    }
}

impl SmtpClient for SmtpTransport {
    fn send(&self, email: &Message) -> Result<(), Error> {
        Transport::send(self, email).map_err(Error::FailedToSendEmail)?;
        Ok(())
    }
}

pub struct Email {
    /// By default, the meta is loaded from `email/config.yaml`.
    /// When this field is not None, it overrides original behaviour
    pub meta: Option<EmailMeta>,
    pub sender_email: String,
    /// All recipients will be able to see each other
    /// Use `bcc` field for hidden recipients
    pub recipients: Vec<String>,
    pub bcc: Vec<String>,
    pub template: Box<dyn EmailTemplate>,
    pub attachments: Vec<EmailAttachment>,
}

impl Email {
    pub fn new(
        local_email_part: &str,
        recipients: Vec<String>,
        template: Box<dyn EmailTemplate>,
    ) -> Self {
        Self::new_with_meta(local_email_part, Vec::new(), recipients, template, None)
    }

    pub fn new_with_meta(
        local_email_part: &str,
        recipients: Vec<String>,
        bcc: Vec<String>,
        template: Box<dyn EmailTemplate>,
        meta: Option<EmailMeta>,
    ) -> Self {
        let full_email = format!("{local_email_part}@{}", &EnvConfig::get().domain);

        Email {
            meta,
            sender_email: full_email,
            recipients,
            bcc,
            template,
            attachments: Vec::new(),
        }
    }

    pub fn with_attachment(mut self, attachment: EmailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub async fn send(&self, smtp_client: &dyn SmtpClient) -> Result<(), Error> {
        let email = self.build_email()?;
        smtp_client.send(&email)?;

        Ok(())
    }

    fn parse_address<T: FromStr>(address: &str) -> Result<T, Error> {
        address
            .parse()
            .map_err(|_| Error::InvalidEmailSender(address.to_string()))
    }

    fn build_email(&self) -> Result<Message, Error> {
        let meta = self.meta();
        let rendered_email = self.template.dyn_render()?;

        let mut email_builder = Message::builder()
            .from(Mailbox::new(
                meta.sender_name,
                Self::parse_address(&self.sender_email)?,
            ))
            .subject(&meta.subject);

        for to in &self.recipients {
            if to == UNDISCLOSED_RECIPIENTS {
                email_builder = email_builder.raw_header(HeaderValue::new(
                    TO_EMAIL_HEADER,
                    UNDISCLOSED_RECIPIENTS.to_string(),
                ));
                continue;
            }
            email_builder = email_builder.to(Self::parse_address(to)?);
        }

        for bcc in &self.bcc {
            email_builder = email_builder.bcc(Self::parse_address(bcc)?);
        }

        let html_content = SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(rendered_email);

        let email = if self.attachments.is_empty() {
            email_builder
                .singlepart(html_content)
                .map_err(Error::FailedToBuildEmail)?
        } else {
            let mut body = MultiPart::mixed().singlepart(html_content);

            for attachment in &self.attachments {
                body = body.singlepart(
                    Attachment::new(attachment.filename.clone())
                        .body(attachment.body.clone(), attachment.content_type.clone()),
                );
            }

            email_builder
                .multipart(body)
                .map_err(Error::FailedToBuildEmail)?
        };

        Ok(email)
    }

    fn meta(&self) -> EmailMeta {
        let file = include_str!("../../templates/email/config.yaml");
        let templates: HashMap<String, EmailMeta> = serde_norway::from_str(file).unwrap();

        match &self.meta {
            Some(meta) => meta.clone(),
            None => templates
                .get(self.template.id())
                .cloned()
                .unwrap_or_default(),
        }
    }
}
