use crate::services::env::EnvConfig;
use crate::utils::email::RawToHeader;
use crate::utils::error::Error;
use askama::{DynTemplate, Template};
use lettre::message::{Mailbox, SinglePart, header};
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::option::Option;
use std::str::FromStr;
use utoipa::ToSchema;

pub const UNDISCLOSED_RECIPIENTS: &str = "undisclosed-recipients:;";

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/email_confirmation.html")]
pub struct EmailConfirmation {
    pub link: String,
    pub user: String,
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/reset_password.html")]
pub struct ResetPassword {
    pub link: String,
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/informational.html")]
pub struct Informational {
    pub title: String,
    pub content: String,
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/external_registration_form.html")]
pub struct ExternalRegistrationForm {
    pub organization: String,
    pub link: String,
}

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

pub struct Email {
    /// By default, the meta is loaded from `email/config.yaml`.
    /// When this field is not None, it overrides original behaviour
    pub meta: Option<EmailMeta>,
    pub email: String,
    /// All recipients will be able to see each other
    /// Use `bcc` field for hidden recipients
    pub recipients: Vec<String>,
    pub bcc: Vec<String>,
    pub template: Box<dyn EmailTemplate>,
}

impl Email {
    pub fn new(email: String, recipients: Vec<String>, template: Box<dyn EmailTemplate>) -> Self {
        Self::new_with_meta(email, Vec::new(), recipients, template, None)
    }

    pub fn new_with_meta(
        email: String,
        recipients: Vec<String>,
        bcc: Vec<String>,
        template: Box<dyn EmailTemplate>,
        meta: Option<EmailMeta>,
    ) -> Self {
        let full_email = format!("{email}@{}", &EnvConfig::get().domain);

        Email {
            meta,
            email: full_email,
            recipients,
            bcc,
            template,
        }
    }

    pub async fn send(&self, smtp_client: &SmtpTransport) -> Result<(), Error> {
        let email = self.build_email()?;
        smtp_client.send(&email).map_err(Error::FailedToSendEmail)?;

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
                Self::parse_address(&self.email)?,
            ))
            .subject(&meta.subject);

        for to in &self.recipients {
            if to == UNDISCLOSED_RECIPIENTS {
                email_builder = email_builder.header(RawToHeader(UNDISCLOSED_RECIPIENTS));
                continue;
            }
            email_builder = email_builder.to(Self::parse_address(to)?);
        }

        for bcc in &self.bcc {
            email_builder = email_builder.bcc(Self::parse_address(bcc)?);
        }

        let email = email_builder
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(rendered_email),
            )
            .map_err(Error::FailedToBuildEmail)?;

        Ok(email)
    }

    fn meta(&self) -> EmailMeta {
        let file = include_str!("../../templates/email/config.yaml");
        let templates: HashMap<String, EmailMeta> = serde_yml::from_str(file).unwrap();

        match &self.meta {
            Some(meta) => meta.clone(),
            None => templates
                .get(self.template.id())
                .cloned()
                .unwrap_or_default(),
        }
    }
}
