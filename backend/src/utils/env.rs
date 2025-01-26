use serde::Deserialize;

fn default_backend_address() -> String {
    "127.0.0.1:8080".to_string()
}
fn default_openapi_json_frontend_path() -> String {
    "../frontend/openapi/api/openapi.json".to_string()
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

pub fn load_config() -> Result<Config, envy::Error> {
    envy::from_env::<Config>()
}
