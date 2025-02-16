use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::HttpResponse;
use lettre::message::{header, Attachment, Mailbox, Mailboxes, MultiPart, SinglePart};
use lettre::{Message, Transport};
use std::option::Option;
use std::path::Path;

pub enum EmailTemplate {
    HelloWorld,
    EmailConfirmation,
}

impl EmailTemplate {
    pub fn get_placeholder_elements(&self) -> Option<Vec<String>> {
        match self {
            EmailTemplate::HelloWorld => None,
            EmailTemplate::EmailConfirmation => Some(vec!["user".to_string(), "link".to_string()]),
        }
    }
    pub fn get_template_path(&self) -> String {
        match self {
            EmailTemplate::HelloWorld => "src/services/emails_assets/hello_world.html".to_string(),
            EmailTemplate::EmailConfirmation => {
                "src/services/emails_assets/email_confirmation.html".to_string()
            }
        }
    }
    pub fn is_logo_attached(&self) -> bool {
        match self {
            EmailTemplate::HelloWorld => false,
            EmailTemplate::EmailConfirmation => true,
        }
    }
}

pub struct Email {
    pub sender: (Option<String>, String),
    pub recipients: Vec<String>,
    pub subject: String,
    pub template: EmailTemplate,
    pub placeholders: Option<Vec<(String, String)>>,
}

impl Email {
    pub async fn send(&self, app_state: &AppState) -> Result<HttpResponse, Error> {
        let smtp_client = &app_state.smtp_client;

        let html = self.parse_placeholders()?;

        let mut email_body = MultiPart::mixed().singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html),
        );

        if self.template.is_logo_attached() {
            let logo_attachment = Attachment::new_inline("hack4krak_logo".to_string()).body(
                tokio::fs::read("src/services/emails_assets/images/hack4krak_logo_black.webp")
                    .await
                    .map_err(|_| Error::EmailAssetsNotFound)?,
                header::ContentType::parse("image/webp").unwrap(),
            );
            email_body = email_body.singlepart(logo_attachment);
        }

        let mailboxes: Mailboxes = self
            .recipients
            .iter()
            .map(|recipient| recipient.parse())
            .collect::<Result<Mailboxes, _>>()
            .map_err(|_| Error::InvalidEmailRecipients(self.recipients.join(", ")))?;

        let to_header: header::To = mailboxes.into();

        let (sender_name, sender_email) = self.sender.clone();

        let sender = Mailbox::new(
            sender_name,
            sender_email
                .parse()
                .map_err(|_| Error::InvalidEmailSender(sender_email))?,
        );

        let email = Message::builder()
            .from(sender)
            .mailbox(to_header)
            .subject(&self.subject)
            .multipart(email_body)
            .map_err(Error::FailedToBuildEmail)?;

        let _email = smtp_client.send(&email).map_err(Error::FailedToSendEmail)?;

        Ok(HttpResponse::Ok().json("Email successfully sent"))
    }

    fn parse_placeholders(&self) -> Result<String, Error> {
        if self.template.get_placeholder_elements().is_some() && self.placeholders.is_none() {
            return Err(Error::PlaceholdersRequired);
        }

        let mut html = std::fs::read_to_string(Path::new(&self.template.get_template_path()))
            .map_err(|_| Error::EmailTemplateNotFound)?;
        if let Some(elements) = self.placeholders.clone() {
            for (key, value) in elements {
                html = html.replace(&format!("%{}%", key), &value);
            }
        }
        Ok(html)
    }
}
