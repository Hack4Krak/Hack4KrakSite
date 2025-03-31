use crate::services::env::EnvConfig;
use crate::utils::cookies::ACCESS_TOKEN_COOKIE;
use crate::utils::error::Error;
use serde_json::to_string;
use std::fs::File;
use std::io::Write;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::openapi::{OpenApi, Server};
use utoipa::{Modify, OpenApi as _};

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Hack4Krak API Documentation",
        license(name = "GPL-3.0", url = "https://www.gnu.org/licenses/gpl-3.0.en.html"),
        version = env!("CARGO_PKG_VERSION")
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn openapi_with_server() -> OpenApi {
        let mut api = Self::openapi();
        let server = Server::new(EnvConfig::get().backend_url.clone().to_string());

        api.servers = Some(vec![server]);

        api
    }
}

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
