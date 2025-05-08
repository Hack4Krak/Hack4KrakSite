use crate::entities::external_team_invitation;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Path;
use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
struct Model {
    team_name: String,
    codes: Vec<String>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully joined team.", body = Vec<Model>),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "Invitation not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "teams/external_invitations"
)]
#[get("/list/{code}")]
pub async fn list(
    app_state: web::Data<app_state::AppState>,
    confirmation_code: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let results =
        external_team_invitation::Model::list(&app_state.database, confirmation_code.into_inner())
            .await?;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (access_code, team_name) in results {
        map.entry(team_name).or_default().push(access_code);
    }

    let grouped: Vec<Model> = map
        .into_iter()
        .map(|(team_name, codes)| Model { team_name, codes })
        .collect();

    Ok(HttpResponse::Ok().json(grouped))
}
