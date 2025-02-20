use crate::utils::error::Error;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "User is admin"),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin"
)]
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("User is admin"))
}
