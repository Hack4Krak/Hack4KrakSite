#![cfg(feature = "full-test-suite")]

use actix_web::web::Data;
use hack4krak_backend::services::emails::{Email, EmailTemplate};
use hack4krak_backend::utils::app_state::AppState;
use lettre::transport::smtp::client::Tls;
use lettre::SmtpTransport;
use serde_json::Value;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::GenericImage;

#[actix_web::test]
async fn send_mail() {
    let container = GenericImage::new("mailhog/mailhog", "latest")
        .with_exposed_port(1025.tcp()) // SMTP port
        .with_exposed_port(8025.tcp()) // HTTP API port
        .with_wait_for(WaitFor::message_on_stdout("Creating API v2 with WebPath:"))
        .start()
        .await
        .unwrap();

    let host = container.get_host().await.unwrap();

    let smtp_port = container.get_host_port_ipv4(1025).await.unwrap();
    let http_port = container.get_host_port_ipv4(8025).await.unwrap();

    let smtp_client = SmtpTransport::builder_dangerous(host.to_string())
        .tls(Tls::None)
        .port(smtp_port)
        .build();

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
        .send(&Data::new(AppState::with_email_client(smtp_client)))
        .await
        .unwrap();

    let api_url = format!("http://{}:{}/api/v2/messages", host, http_port);
    let response = reqwest::get(&api_url)
        .await
        .expect("Failed to call MailHog API")
        .json::<Value>()
        .await
        .expect("Failed to parse API response");

    let count = response["count"].as_u64().expect("Invalid count format");
    assert_eq!(count, 1, "No email found in MailHog");

    let items = response["items"].as_array().unwrap();
    let first_email = &items[0];

    assert_eq!(
        first_email["Raw"]["From"].as_str().unwrap(),
        "user@domain.tld"
    );

    assert_eq!(
        first_email["Raw"]["To"][0].as_str().unwrap(),
        "danger@domain.tld"
    );

    assert_eq!(
        first_email["Raw"]["To"][1].as_str().unwrap(),
        "dziegiel@domain.tld"
    );

    assert_eq!(
        first_email["Raw"]["To"][2].as_str().unwrap(),
        "borbert@domain.tld"
    );
}
