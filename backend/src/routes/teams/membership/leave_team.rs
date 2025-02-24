use crate::entities::{teams, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, delete, web};

#[utoipa::path(
    responses(
        (status = 200, description = "User successfully left team."),
        (status = 403, description = "User is not a member of the team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/membership"
)]
#[delete("/leave_team", wrap = "AuthMiddleware::with_team_as_member()")]
pub async fn leave_team(
    app_state: web::Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    teams::Model::remove_user(&app_state.database, user).await?;

    Ok(SuccessResponse::default().http_response())
}
