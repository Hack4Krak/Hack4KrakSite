use crate::routes::admin::EmailSendingModel;
use crate::utils::error::Error;
use lettre::message::{Mailbox, Mailboxes, header};
use lettre::{Message, SmtpTransport, Transport};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::option::Option;
use askama::{DynTemplate, Template};
use utoipa::ToSchema;
use crate::services::env::EnvConfig;

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/email_confirmation.html", blocks = ["meta"])]
pub struct EmailConfirmation {
    pub link: String,
    pub user: String,
}

impl EmailTemplate for EmailConfirmation {
    fn meta_template(&self) -> Box<dyn DynTemplate + '_> {
        Box::new(self.as_meta())
    }
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/reset_password.html", blocks = ["meta"])]
pub struct ResetPassword {
    pub link: String,
}

impl EmailTemplate for ResetPassword {
    fn meta_template(&self) -> Box<dyn DynTemplate + '_> {
        Box::new(self.as_meta())
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct EmailMeta {
    pub subject: String,
    pub sender: Option<String>,
}

pub trait EmailTemplate: DynTemplate {
    fn meta_template(&self) -> Box<dyn DynTemplate + '_>;

    fn meta(&self) -> Result<EmailMeta, Error> {
        Ok(serde_yml::from_str::<EmailMeta>(&self.meta_template().dyn_render()?)?)
    }
}


pub struct Email {
    /// By default, the meta is loaded from email template.
    /// You can use this option to override this behaviour
    pub meta: Option<EmailMeta>,
    pub email: String,
    pub recipients: Vec<String>,
    pub template: Box<dyn EmailTemplate>,
}

impl Email {
    pub fn new(email: String, recipients: Vec<String>, template: Box<dyn EmailTemplate>) -> Self {
        let full_email = format!("{email}@{}", &EnvConfig::get().domain);
        
        Email {
            meta: None,
            email: full_email,
            recipients,
            template,
        }
    }
    
    pub async fn send(&self, smtp_client: &SmtpTransport) -> Result<(), Error> {
        let meta = match &self.meta {
            Some(meta) => Ok(meta.clone()),
            None => self.template.meta(),
        }?;
        let rendered_email = self.template.dyn_render()?;

        let mailboxes: Mailboxes = self
            .recipients
            .iter()
            .map(|recipient| recipient.parse())
            .collect::<Result<Mailboxes, _>>()
            .map_err(|_| Error::InvalidEmailRecipients(self.recipients.join(", ")))?;

        let sender = Mailbox::new(
            meta.sender,
            self.email
                .parse()
                .map_err(|_| Error::InvalidEmailSender(self.email.clone()))?,
        );

        let email = Message::builder()
            .from(sender)
            .mailbox(header::To::from(mailboxes))
            .subject(&meta.subject)
            .body(rendered_email)
            .map_err(Error::FailedToBuildEmail)?;

        smtp_client.send(&email).map_err(Error::FailedToSendEmail)?;

        Ok(())
    }

    pub async fn from_admin_sending_model(
        database: &DatabaseConnection,
        model: EmailSendingModel,
    ) -> Result<Self, Error> {
        unimplemented!()
        // let mut recipients_emails = Vec::new();
        // if let Some(recipients) = model.recipients {
        //     for recipient in recipients {
        //         let user = users::Model::find_by_username(database, &recipient).await?;
        //         recipients_emails.push(
        //             user.ok_or(Error::RecipientNotFound {
        //                 username: recipient,
        //             })?
        //             .email,
        //         );
        //     }
        // } else {
        //     let users = users::Entity::find().all(database).await?;
        //     recipients_emails.extend(users.into_iter().map(|user| user.email));
        // }
        //
        // // Ok(Self {
        // //     sender: model.sender,
        // //     recipients: recipients_emails,
        // //     subject: model.subject,
        // //     // template: model.template,
        // //     // placeholders: model.placeholders,
        // // })
    }
}
