use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use actix_web::middleware::from_fn;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddUserModel {
    pub username: String,
}

#[utoipa::path(
    request_body = AddUserModel,
    responses(
        (status = 200, description = "User successfully invited to team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[post(
    "/manage/invite_user",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn invite_user(
    app_state: web::Data<app_state::AppState>,
    add_user_model: web::Json<AddUserModel>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    teams::Model::invite_user(&app_state.database, add_user_model.0, claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct InvitationsResponse {
    pub invitations_vector: Vec<String>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Sucessfully retrievied invitations", body = InvitationsResponse),
        (status = 404, description = "User doesn't have any invitations."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[get(
    "/manage/invitations",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
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
            .unwrap();
        invitations_vector.push(team.name)
    }

    Ok(HttpResponse::Ok().json(InvitationsResponse { invitations_vector }))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully joined team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "Invitation not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[post(
    "/manage/accept_invitation/{team_name}",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn accept_invitation(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    team_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    teams::Model::accept_invitation(&app_state.database, team_name.to_string(), claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
