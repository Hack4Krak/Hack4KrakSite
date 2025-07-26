use crate::entities::users;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::cookies::REFRESH_TOKEN_COOKIE;
use crate::utils::error::Error;
use crate::utils::jwt::decode_jwt;
use actix_web::{HttpRequest, HttpResponse, post, web};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "New tokens are set as cookies"),
        (status = 401, description = "Invalid credentials."),
    ),
    security(
        ("access_token" = [])
    ),
    tag = "auth"
)]
#[post("/refresh")]
pub async fn refresh(
    app_state: web::Data<app_state::AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let Some(refresh_token) = request.cookie(REFRESH_TOKEN_COOKIE) else {
        return Err(Error::Unauthorized);
    };

    let claims = decode_jwt(refresh_token.value()).map_err(|_| Error::Unauthorized)?;
    users::Entity::find_by_id(claims.claims.id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Unauthorized)?;

    let response = AuthService::response_with_cookies(claims.claims.id, claims.claims.email)?;
    Ok(response)
}
