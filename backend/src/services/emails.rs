use crate::entities::users;
use crate::routes::admin::EmailSendingModel;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::HttpResponse;
use lettre::message::{Attachment, Mailbox, Mailboxes, MultiPart, SinglePart, header};
use lettre::{Message, Transport};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::option::Option;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum EmailTemplate {
    HelloWorld,
    EmailConfirmation,
    Generic,
    ExternalRegistrationForm,
}

impl EmailTemplate {
    pub fn get_placeholder_elements(&self) -> Option<Vec<String>> {
        match self {
            EmailTemplate::HelloWorld => None,
            EmailTemplate::EmailConfirmation => Some(vec!["user".to_string(), "link".to_string()]),
            EmailTemplate::Generic => Some(vec!["body".to_string()]),
            EmailTemplate::ExternalRegistrationForm => {
                Some(vec!["organization".to_string(), "link".to_string()])
            }
        }
    }

    pub fn get_template(&self) -> &'static str {
        match self {
            EmailTemplate::HelloWorld => include_str!("emails_assets/hello_world.html"),
            EmailTemplate::EmailConfirmation => {
                include_str!("emails_assets/email_confirmation.html")
            }
            EmailTemplate::Generic => include_str!("emails_assets/generic.html"),
            &EmailTemplate::ExternalRegistrationForm => {
                include_str!("emails_assets/external_registration_form.html")
            }
        }
    }

    pub fn is_logo_attached(&self) -> bool {
        match self {
            EmailTemplate::HelloWorld => false,
            EmailTemplate::EmailConfirmation => true,
            EmailTemplate::Generic => true,
            EmailTemplate::ExternalRegistrationForm => false,
        }
    }

    pub fn list() -> Vec<Self> {
        vec![
            Self::HelloWorld,
            Self::EmailConfirmation,
            Self::Generic,
            Self::ExternalRegistrationForm,
        ]
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
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

    pub async fn from_admin_sending_model(
        database: &DatabaseConnection,
        model: EmailSendingModel,
    ) -> Result<Self, Error> {
        let mut recipients_emails = Vec::new();
        if let Some(recipients) = model.recipients {
            for recipient in recipients {
                let user = users::Model::find_by_username(database, &recipient).await?;
                recipients_emails.push(
                    user.ok_or(Error::RecipientNotFound {
                        username: recipient,
                    })?
                    .email,
                );
            }
        } else {
            let users = users::Entity::find().all(database).await?;
            recipients_emails.extend(users.into_iter().map(|user| user.email));
        }

        Ok(Self {
            sender: model.sender,
            recipients: recipients_emails,
            subject: model.subject,
            template: model.template,
            placeholders: model.placeholders,
        })
    }

    fn parse_placeholders(&self) -> Result<String, Error> {
        if self.template.get_placeholder_elements().is_some() && self.placeholders.is_none() {
            return Err(Error::PlaceholdersRequired);
        }

        let mut html = self.template.get_template().to_string();
        if let Some(elements) = self.placeholders.clone() {
            for (key, value) in elements {
                html = html.replace(&format!("%{}%", key), &value);
            }
        }
        Ok(html)
    }
}
