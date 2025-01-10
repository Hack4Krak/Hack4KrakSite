use crate::utils::error::Error;
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
