use std::env;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

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

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "access_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
