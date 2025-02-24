use crate::entities::team_invites;
use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved invitations to team"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("access_token" = ["leader"]),
    ),
    tag = "teams/management"
)]
#[get("/invited_users")]
pub async fn invited_users(
    app_state: Data<app_state::AppState>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    let users = team_invites::Model::get_invited_users(&app_state.database, team).await?;

    Ok(HttpResponse::Ok().json(users))
}
