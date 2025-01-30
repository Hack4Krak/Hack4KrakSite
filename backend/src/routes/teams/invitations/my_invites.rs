use crate::models::entities::teams;
use crate::routes::teams::TeamError::TeamNotFound;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use actix_web::{get, web, HttpResponse};
use sea_orm::EntityTrait;

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
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    let invitations = teams::Model::get_invitations(&app_state.database, claim_data).await?;
    let mut invitations_vector = Vec::new();

    for invitation in invitations {
        let team = teams::Entity::find_by_id(invitation.team)
            .one(&app_state.database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;
        invitations_vector.push(team.name)
    }

    Ok(HttpResponse::Ok().json(invitations_vector))
}
