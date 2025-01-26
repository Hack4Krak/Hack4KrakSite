use serde::Deserialize;
fn default_database_url() -> String {
    "postgres://backend:password@localhost:5432/hack4krak".to_string()
}
fn default_backend_address() -> String {
    "127.0.0.1:8080".to_string()
}
fn default_openapi_json_frontend_path() -> String {
    "../frontend/openapi/api/openapi.json".to_string()
}
fn default_jwt_secret() -> String {
    "a-string-secret-at-least-256-bits-long".to_string()
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    #[serde(default = "default_backend_address")]
    pub backend_address: String,
    #[serde(default = "default_openapi_json_frontend_path")]
    pub openapi_json_frontend_path: String,
    pub jwt_secret: String,
    pub github_oauth_client_id: String,
    pub github_oauth_client_secret: String,
}

impl Config {
    pub fn from_dumb_data() -> Config {
        Config {
            database_url: default_database_url(),
            backend_address: default_backend_address(),
            openapi_json_frontend_path: default_openapi_json_frontend_path(),
            jwt_secret: default_jwt_secret(),
            github_oauth_client_id: "test".to_string(),
            github_oauth_client_secret: "test".to_string(),
        }
    }
}

pub fn load_config() -> Result<Config, envy::Error> {
    envy::from_env::<Config>()
}
