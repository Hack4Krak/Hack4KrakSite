use actix_web::{HttpResponse, get};
use serde::Serialize;

use crate::utils::error::Error;

#[derive(Serialize, utoipa::ToSchema)]
struct ResponseData<'a> {
    version: &'a str,
    name: &'a str,
    about: &'a str,
}

#[utoipa::path(responses((status = OK, body = ResponseData)))]
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(ResponseData {
        version: env!("CARGO_PKG_VERSION"),
        name: "Hack4Krak",
        about: "API for https://github.com/Hack4Krak/Hack4KrakSite",
    }))
}
