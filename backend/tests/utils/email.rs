#![cfg(feature = "full-test-suite")]

use crate::test_utils::mail::SmtpTestClient;
use hack4krak_backend::services::emails;
use hack4krak_backend::utils::email::{Email, EmailMeta};

#[actix_web::test]
async fn send_mail() {
    let mail_client = SmtpTestClient::new().await;

    let email = Email {
        meta: Some(EmailMeta {
            subject: "Le Corbusier and his crimes".to_string(),
            sender_name: None,
        }),
        recipients: vec![
            "danger@domain.tld".to_string(),
            "dziegiel@domain.tld".to_string(),
            "borbert@domain.tld".to_string(),
        ],
        bcc: Vec::new(),
        template: Box::new(emails::Informational {
            title: "Important message".to_string(),
            content: "Le Corbusier and his crimes".to_string(),
        }),
        sender_email: "user@domain.tld".to_string(),
    };

    email.send(&mail_client.smtp_client).await.unwrap();

    let emails = mail_client.get_emails().await;
    assert_eq!(emails.count, 1, "No email found in MailHog");

    let first_email = &emails.items[0];

    assert_eq!(first_email.raw.from.as_str(), "user@domain.tld");

    assert_eq!(first_email.raw.to[0].as_str(), "danger@domain.tld");
    assert_eq!(first_email.raw.to[1].as_str(), "dziegiel@domain.tld");
    assert_eq!(first_email.raw.to[2].as_str(), "borbert@domain.tld");
}
