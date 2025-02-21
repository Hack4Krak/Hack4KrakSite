use crate::entities::{teams, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Data;
use actix_web::{delete, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Team deleted successfully"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("access_token" = ["leader"])
    ),
    tag = "teams/management",
)]
#[delete("/delete", wrap = "AuthMiddleware::with_team_as_leader()")]
pub async fn delete_team(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    teams::Model::delete(&app_state.database, user, team).await?;

    Ok(SuccessResponse::default().http_response())
}
