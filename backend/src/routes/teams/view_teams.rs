use crate::models::entities::{teams, users};
use crate::routes::teams::view_team::TeamWithMembers;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{get, web, HttpResponse};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "Teams successfully retrieved.", body = Vec<TeamWithMembers>),
        (status = 500, description = "Internal server error.")
    )
)]
#[get("/")]
pub async fn view_teams(app_state: web::Data<app_state::AppState>) -> Result<HttpResponse, Error> {
    let teams = teams::Entity::find()
        .find_also_related(users::Entity)
        .all(&app_state.database)
        .await?;

    let teams_with_members = teams
        .into_iter()
        .map(|(team, users)| {
            let member_names: Vec<String> = users.into_iter().map(|user| user.username).collect();
            TeamWithMembers {
                team_name: team.name,
                leader_name: team.leader_name,
                created_at: team.created_at,
                members: member_names,
            }
        })
        .collect::<Vec<TeamWithMembers>>();

    Ok(HttpResponse::Ok().json(teams_with_members))
}
