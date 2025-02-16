use actix_web::{HttpResponse, HttpResponseBuilder};

pub fn create_redirect_response(location: String) -> HttpResponseBuilder {
    let mut response = HttpResponse::Ok();
    response.append_header(("Refresh", format!("0; {}", location)));
    response
}
