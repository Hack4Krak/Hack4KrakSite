#![cfg(feature = "full-test-suite")]

use crate::test_utils::mail::SmtpTestClient;
use actix_web::web::Data;
use hack4krak_backend::services::emails::{Email, EmailTemplate};
use hack4krak_backend::utils::app_state::AppState;

#[actix_web::test]
async fn send_mail() {
    let mail_client = SmtpTestClient::new().await;

    let email = Email {
        sender: (None, "user@domain.tld".to_string()),
        subject: "Le Corbusier and his crimes".to_string(),
        recipients: vec![
            "danger@domain.tld".to_string(),
            "dziegiel@domain.tld".to_string(),
            "borbert@domain.tld".to_string(),
        ],
        template: EmailTemplate::HelloWorld,
        placeholders: None,
    };

    email
        .send(&Data::new(AppState::with_email_client(
            mail_client.smtp_client.clone(),
        )))
        .await
        .unwrap();

    let emails = mail_client.get_emails().await;
    assert_eq!(emails.count, 1, "No email found in MailHog");

    let first_email = &emails.items[0];

    assert_eq!(first_email.raw.from.as_str(), "user@domain.tld");

    assert_eq!(first_email.raw.to[0].as_str(), "danger@domain.tld");
    assert_eq!(first_email.raw.to[1].as_str(), "dziegiel@domain.tld");
    assert_eq!(first_email.raw.to[2].as_str(), "borbert@domain.tld");
}
