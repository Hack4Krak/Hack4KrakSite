use crate::utils::error::Error;
use actix_web::{HttpResponse, ResponseError};

pub async fn not_found() -> HttpResponse {
    Error::RouteNotFound.error_response()
}
