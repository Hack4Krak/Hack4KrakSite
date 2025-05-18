use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;
use std::str::FromStr;
use url::Url;

pub static ENV: OnceLock<EnvConfig> = OnceLock::new();

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct EnvConfig {
    pub database_url: String,
    pub backend_address: String,
    pub domain: String,
    pub backend_url: Url,
    pub frontend_url: Url,
    pub openapi_json_frontend_path: String,
    pub cookies_domain: String,
    pub jwt_secret: String,
    pub tasks_base_path: PathBuf,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub resend_api_key: String,
    pub metrics_access_token: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        EnvConfig {
            backend_address: "127.0.0.1:8080".to_string(),
            domain: "hack4krak.pl".to_string(),
            backend_url: Url::parse("http://localhost:8080").unwrap(),
            frontend_url: Url::parse("http://localhost:3000").unwrap(),
            openapi_json_frontend_path: "../frontend/openapi/api/openapi.json".to_string(),
            cookies_domain: "localhost".to_string(),
            tasks_base_path: PathBuf::from_str("TasksTemplate/").unwrap(),
            database_url: Default::default(),
            github_oauth_client_id: Default::default(),
            github_oauth_client_secret: Default::default(),
            google_oauth_client_id: Default::default(),
            google_oauth_client_secret: Default::default(),
            jwt_secret: Default::default(),
            resend_api_key: Default::default(),
            metrics_access_token: Default::default(),
        }
    }
}

impl EnvConfig {
    pub fn load_config() {
        let _ = dotenvy::from_path(Path::new("../.env"));
        ENV.get_or_init(|| envy::from_env::<EnvConfig>().unwrap());
    }

    pub fn load_test_config() {
        ENV.get_or_init(|| EnvConfig {
            jwt_secret: "skibidi-dziegiel-secret".to_string(),
            ..Default::default()
        });
    }

    pub fn get() -> &'static EnvConfig {
        ENV.get().unwrap()
    }

    pub fn get_ip_and_port(&self) -> (&str, u16) {
        let address_env = &EnvConfig::get().backend_address;
        let address_vec: Vec<&str> = address_env.split(":").collect();
        let ip = address_vec[0];
        let port = address_vec[1]
            .parse::<u16>()
            .expect("The port in BACKEND_ADDRESS must be a valid u16 integer");

        (ip, port)
    }
}
