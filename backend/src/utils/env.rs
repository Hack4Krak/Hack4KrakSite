use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::Path;
use Default;

fn default_backend_address() -> String {
    "127.0.0.1:8080".to_string()
}
fn default_openapi_json_frontend_path() -> String {
    "../frontend/openapi/api/openapi.json".to_string()
}

lazy_static! {
    pub static ref CONFIG: Config = {
        if cfg!(test) {
            Config::load_test_config()
        } else {
            Config::load_config().unwrap()
        }
    };
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_backend_address")]
    pub backend_address: String,
    #[serde(default = "default_openapi_json_frontend_path")]
    pub openapi_json_frontend_path: String,
    pub jwt_secret: String,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
    pub github_oauth_redirect_url: String,
}

impl Config {
    pub fn load_config() -> Result<Config, envy::Error> {
        dotenvy::from_path(Path::new("../.env")).unwrap();
        envy::from_env::<Config>()
    }

    pub fn load_test_config() -> Config {
        Config {
            jwt_secret: "skibidi-dziegiel-secret".to_string(),
            ..Default::default()
        }
    }
}
