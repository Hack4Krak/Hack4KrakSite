use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::HttpResponse;
use lettre::message::{header, Attachment, Mailbox, Mailboxes, MultiPart, SinglePart};
use lettre::{Message, Transport};
use std::collections::HashMap;
use std::option::Option;
use text_template::Template;

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
    pub fn get_template(&self) -> &'static str {
        match self {
            EmailTemplate::HelloWorld => include_str!("emails_assets/hello_world.html"),
            EmailTemplate::EmailConfirmation => {
                include_str!("emails_assets/email_confirmation.html")
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

pub struct Email<'a> {
    pub sender: (Option<String>, String),
    pub recipients: Vec<String>,
    pub subject: String,
    pub template: EmailTemplate,
    pub placeholders: Option<HashMap<&'a str, &'a str>>,
}

impl Email<'_> {
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
                include_bytes!("emails_assets/images/hack4krak_logo_black.webp").to_vec(),
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

        if self.template.get_placeholder_elements().is_none() {
            let html = self.template.get_template().to_string();

            return Ok(html);
        }

        let template = Template::from(self.template.get_template());
        let values = self.placeholders.clone().unwrap();

        let html = template.fill_in(&values);

        Ok(html.to_string())
    }
}
