use crate::utils::error::Error;
use actix_web::{HttpResponse, HttpResponseBuilder, http::header::LOCATION};
use url::Url;

pub fn create_redirect_response(location: String) -> Result<HttpResponseBuilder, Error> {
    let mut response = HttpResponse::Ok();
    let url = Url::parse(location.as_str()).map_err(Error::FailedToParseUrl)?;
    response.append_header(("Refresh", format!("0; {}", url)));
    Ok(response)
}

pub fn create_temporary_redirect_response(
    location: String,
    error_message: Error,
) -> Result<HttpResponseBuilder, Error> {
    let mut response = HttpResponse::TemporaryRedirect();
    let url = Url::parse_with_params(location.as_str(), &[("error", error_message.to_string())])
        .map_err(Error::FailedToParseUrl)?;
    response.append_header((LOCATION, url.to_string()));
    Ok(response)
}
