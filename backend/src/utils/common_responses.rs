use actix_web::{HttpResponse, HttpResponseBuilder};
use crate::utils::error::Error;

pub fn create_redirect_response(location: String) -> HttpResponseBuilder {
    let mut response = HttpResponse::Ok();
    response.append_header(("Refresh", format!("0; {}", location)));
    response
}

pub fn create_temporary_redirect_response(location: String, error_message: Error) -> HttpResponseBuilder {
    let mut response = HttpResponse::TemporaryRedirect();
    response.append_header(("Location", format!("{}?error={}", location, error_message)));
    response
}
