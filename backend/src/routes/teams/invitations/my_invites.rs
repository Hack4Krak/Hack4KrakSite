use crate::entities::{team_invites, users};
use crate::routes::teams::TeamError::TeamNotFound;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved invitations", body = Vec<String>),
        (status = 404, description = "User doesn't have any invitations."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/invitations"
)]
#[get("/")]
pub async fn get_invitations(
    app_state: web::Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let invitations =
        team_invites::Model::get_invitations_with_teams(&app_state.database, user).await?;

    let mut invitations_output = Vec::new();
    for (_, team) in invitations {
        let team = team.ok_or(Error::Team(TeamNotFound))?;
        invitations_output.push(team.name)
    }

    Ok(HttpResponse::Ok().json(invitations_output))
}
