use crate::services::emails::EmailTemplate;
use crate::utils::error::Error;
use actix_web::web::Path;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Placeholders successfully retrievied", body = Vec<String>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/email"
)]
#[get("/get_placeholders/{template}")]
pub async fn get_placeholders(template: Path<EmailTemplate>) -> Result<HttpResponse, Error> {
    let placeholders = template.get_placeholder_elements().unwrap_or_default();

    Ok(HttpResponse::Ok().json(placeholders))
}
