use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::{web, HttpResponse};
use lettre::message::header::ContentType;
use lettre::{Message, Transport};
use std::option::Option;
use std::path::Path;

pub enum EmailTemplate {
    HelloWorld,
}

impl EmailTemplate {
    pub fn get_placeholder_elements(&self) -> Option<Vec<String>> {
        match self {
            EmailTemplate::HelloWorld => None,
        }
    }
    pub fn get_template_path(&self) -> String {
        match self {
            EmailTemplate::HelloWorld => "src/utils/emails_assets/hello_world.html".to_string(),
        }
    }
}

pub struct Email {
    pub sender: String,
    pub receivers: Vec<String>,
    pub subject: String,
    pub template: EmailTemplate,
    pub placeholders: Option<Vec<(String, String)>>,
}

impl Email {
    pub async fn send(&self, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
        let smtp_client = &app_state.smtp_client;

        let html = self.parse_placeholders()?;

        let email = Message::builder()
            .from(
                self.sender
                    .parse()
                    .map_err(|_| Error::InvalidEmailAddressSendingEmail)?,
            )
            .to(self
                .receivers
                .join(", ")
                .parse()
                .map_err(|_| Error::InvalidEmailAddressSendingEmail)?)
            .subject(&self.subject)
            .header(ContentType::TEXT_HTML)
            .body(html)
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
