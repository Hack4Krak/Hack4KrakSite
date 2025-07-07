use crate::utils::error::Error;
use actix_web::{HttpResponse, HttpResponseBuilder, http::header::LOCATION};
use url::Url;

pub fn create_redirect_response(location: Url) -> Result<HttpResponseBuilder, Error> {
    let mut response = HttpResponse::Ok();
    response.append_header(("Refresh", format!("0; {location}")));
    Ok(response)
}

pub fn create_temporary_redirect_response(
    location: Url,
    error_message: Error,
) -> Result<HttpResponseBuilder, Error> {
    let mut response = HttpResponse::TemporaryRedirect();
    let url = Url::parse_with_params(location.as_str(), &[("error", error_message.to_string())])?;
    response.append_header((LOCATION, url.to_string()));
    Ok(response)
}
