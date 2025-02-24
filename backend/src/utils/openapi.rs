use crate::services::env::EnvConfig;
use crate::utils::cookies::ACCESS_TOKEN_COOKIE;
use crate::utils::error::Error;
use serde_json::to_string;
use std::fs::File;
use std::io::Write;
use utoipa::Modify;
use utoipa::openapi::OpenApi;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Hack4Krak API Documentation",
        license(name = "GPL-3.0", url = "https://www.gnu.org/licenses/gpl-3.0.en.html"),
        version = env!("CARGO_PKG_VERSION")
    ),
    modifiers(&SecurityAddon),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api.hack4krak.pl", description = "Main API server"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "access_token",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new(ACCESS_TOKEN_COOKIE))),
        )
    }
}

pub fn write_openapi(api: &OpenApi) -> Result<(), Error> {
    // There is no need to generate OpenApi for frontend
    // In test or production environments
    if !cfg!(debug_assertions) {
        return Ok(());
    }

    let path = &EnvConfig::get().openapi_json_frontend_path;
    let mut openapi_json = File::create(path)?;
    openapi_json.write_all(to_string(&api)?.as_bytes())?;

    Ok(())
}
