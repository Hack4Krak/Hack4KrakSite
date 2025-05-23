use lettre::SmtpTransport;
use lettre::transport::smtp::client::Tls;
use quoted_printable::decode;
use serde::{Deserialize, Deserializer, Serialize};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage};
use url::Host;

pub struct SmtpTestClient {
    pub container: ContainerAsync<GenericImage>,
    pub http_port: u16,
    pub smtp_client: SmtpTransport,
    pub host: Host,
}

impl SmtpTestClient {
    pub async fn new() -> Self {
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

        Self {
            container,
            http_port,
            smtp_client,
            host,
        }
    }

    pub async fn get_emails(&self) -> MailhogResponse {
        let api_url = format!("http://{}:{}/api/v2/messages", self.host, self.http_port);
        reqwest::get(&api_url)
            .await
            .expect("Failed to call MailHog API")
            .json::<MailhogResponse>()
            .await
            .expect("Failed to parse API response")
    }

    pub async fn find_uuid_in_first_email(&self) -> String {
        self.get_emails().await.items[0].find_uuid()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailhogResponse {
    pub count: usize,
    pub items: Vec<MailhogItem>,
}

impl MailhogItem {
    pub fn find_uuid(&self) -> String {
        const UUID_REGEX: &str =
            r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}";
        let regex = regex::Regex::new(UUID_REGEX).unwrap();

        let email_body = &self.content.body;
        regex.find(email_body).unwrap().as_str().to_owned()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailhogItem {
    #[serde(rename = "Content")]
    pub content: MailhogContent,
    #[serde(rename = "Raw")]
    pub raw: MailhogRaw,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailhogContent {
    #[serde(rename = "Body", deserialize_with = "decode_quoted_printable")]
    pub body: String,
}

fn decode_quoted_printable<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded: String = Deserialize::deserialize(deserializer)?;
    let decoded =
        decode(&encoded, quoted_printable::ParseMode::Robust).map_err(serde::de::Error::custom)?;
    String::from_utf8(decoded).map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailhogRaw {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: Vec<String>,
}
