use crate::entities::{flag_capture, teams};
use crate::routes::flag::AuthMiddleware;
use crate::routes::flag::FlagError;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SubmitModel {
    #[validate(length(max = 1024))]
    pub flag: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SubmitResponse {
    pub task_id: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly submitted flag.", body = SubmitResponse),
        (status = 400, description = "Invalid flag"),
    ),
    tag = "flag"
)]
#[post("/submit", wrap = "AuthMiddleware::with_team_as_member()")]
pub async fn submit(
    app_state: Data<AppState>,
    model: Validated<Json<SubmitModel>>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    let flag = model
        .flag
        .strip_prefix("hack4KrakCTF{")
        .and_then(|value| value.strip_suffix("}"));

    let Some(flag) = flag else {
        return Err(FlagError::InvalidFlagFormat.into());
    };

    let task = app_state
        .task_manager
        .find_by_flag(flag)
        .ok_or(Error::Flag(FlagError::InvalidFlag))?;

    flag_capture::Model::completed(&app_state.database, team, task.key().to_string()).await?;

    Ok(HttpResponse::Ok().json(SubmitModel {
        flag: task.key().clone(),
    }))
}
