use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;
use std::str::FromStr;
use Default;

fn default_backend_address() -> String {
    "127.0.0.1:8080".to_string()
}
fn default_oauth_finish_redirect_url() -> String {
    "http://localhost:3000/panel".to_string()
}
fn default_email_confirm_redirect_url() -> String {
    "http://localhost:3000/login?redirect_from_confirmation=true".to_string()
}
fn default_email_confirm_backend_url() -> String {
    "http://localhost:8080/auth/confirm".to_string()
}
fn default_openapi_json_frontend_path() -> String {
    "../frontend/openapi/api/openapi.json".to_string()
}
fn default_cookies_domain() -> String {
    "localhost".to_string()
}
fn default_tasks_base_path() -> PathBuf {
    PathBuf::from_str("TasksTemplate/tasks/").unwrap()
}

pub static ENV: OnceLock<EnvConfig> = OnceLock::new();

#[derive(Deserialize, Debug, Default)]
pub struct EnvConfig {
    pub database_url: String,
    #[serde(default = "default_backend_address")]
    pub backend_address: String,
    #[serde(default = "default_oauth_finish_redirect_url")]
    pub oauth_finish_redirect_url: String,
    #[serde(default = "default_openapi_json_frontend_path")]
    pub openapi_json_frontend_path: String,
    #[serde(default = "default_email_confirm_redirect_url")]
    pub email_confirm_redirect_url: String,
    #[serde(default = "default_email_confirm_backend_url")]
    pub email_confirm_backend_url: String,
    #[serde(default = "default_cookies_domain")]
    pub cookies_domain: String,
    pub jwt_secret: String,
    #[serde(default = "default_tasks_base_path")]
    pub tasks_base_path: PathBuf,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
    pub github_oauth_redirect_url: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
    pub resend_api_key: String,
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
