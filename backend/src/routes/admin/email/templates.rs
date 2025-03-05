use crate::services::emails::EmailTemplate;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Templates successfully retrievied", body = Vec<EmailTemplate>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/email"
)]
#[get("/get_templates")]
pub async fn templates() -> Result<HttpResponse, Error> {
    let templates = EmailTemplate::list();
    Ok(HttpResponse::Ok().json(templates))
}
