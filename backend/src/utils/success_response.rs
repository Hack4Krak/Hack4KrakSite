use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse {
    code: u16,
    message: String,
}

impl SuccessResponse {
    pub fn http_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl Default for SuccessResponse {
    fn default() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: "Successfully completed operation!".to_string(),
        }
    }
}
