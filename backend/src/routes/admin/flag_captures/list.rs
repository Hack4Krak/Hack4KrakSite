use crate::entities::flag_capture;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Flag captures successfully fetched.", body = Vec<flag_capture::Model>),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/flag_captures"
)]
#[get("/list")]
pub async fn list(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let flag_captures = flag_capture::Entity::find()
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(flag_captures))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Flag captures for team successfully fetched.", body = Vec<flag_capture::Model>),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/flag_captures"
)]
#[get("/list/team/{team_id}")]
pub async fn list_for_team(
    app_state: Data<AppState>,
    team_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let flag_captures = flag_capture::Entity::find()
        .filter(flag_capture::Column::Team.eq(team_id.into_inner()))
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(flag_captures))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Flag captures for task successfully fetched.", body = Vec<flag_capture::Model>),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/flag_captures"
)]
#[get("/list/task/{task_id}")]
pub async fn list_for_task(
    app_state: Data<AppState>,
    task_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let flag_captures = flag_capture::Entity::find()
        .filter(flag_capture::Column::Task.eq(task_id.into_inner()))
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(flag_captures))
}
