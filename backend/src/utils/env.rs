use std::path::Path;
use std::sync::OnceLock;

use serde::Deserialize;
use Default;

fn default_backend_address() -> String {
    "127.0.0.1:8080".to_string()
}
fn default_oauth_finish_redirect_url() -> String {
    "http://localhost:3000/panel".to_string()
}
fn default_openapi_json_frontend_path() -> String {
    "../frontend/openapi/api/openapi.json".to_string()
}
fn default_cookies_domain() -> String {
    "localhost".to_string()
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_backend_address")]
    pub backend_address: String,
    #[serde(default = "default_oauth_finish_redirect_url")]
    pub oauth_finish_redirect_url: String,
    #[serde(default = "default_openapi_json_frontend_path")]
    pub openapi_json_frontend_path: String,
    #[serde(default = "default_cookies_domain")]
    pub cookies_domain: String,
    pub jwt_secret: String,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
    pub github_oauth_redirect_url: String,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
    pub resend_api_key: String,
}

impl Config {
    pub fn load_config() {
        let _ = dotenvy::from_path(Path::new("../.env"));
        CONFIG.get_or_init(|| envy::from_env::<Config>().unwrap());
    }

    pub fn load_test_config() {
        CONFIG.get_or_init(|| Config {
            jwt_secret: "skibidi-dziegiel-secret".to_string(),
            ..Default::default()
        });
    }

    pub fn get() -> &'static Config {
        CONFIG.get().unwrap()
    }
}
