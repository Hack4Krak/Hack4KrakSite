use actix_http::header;
use actix_http::header::{HeaderName, HeaderValue, TryIntoHeaderPair};
use actix_web::error::HttpError;
use chrono::Duration;
use hack4krak_backend::entities::users;
use hack4krak_backend::utils::jwt::encode_jwt;

pub struct TestAuthHeader {
    user: users::Model,
}

impl TestAuthHeader {
    pub fn new(user: users::Model) -> Self {
        Self { user }
    }
}

impl TryIntoHeaderPair for TestAuthHeader {
    type Error = HttpError;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let access_token = encode_jwt(self.user.id, self.user.email, Duration::minutes(10))
            .map_err(|e| e.to_string())
            .unwrap();

        let header_value = format!("access_token={access_token}");
        Ok((
            header::COOKIE,
            HeaderValue::from_str(&header_value)
                .map_err(|e| e.to_string())
                .unwrap(),
        ))
    }
}
