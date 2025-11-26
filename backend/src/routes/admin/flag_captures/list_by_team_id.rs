use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;
// I need to import it directly, so I can name it differently to make OpenAPI generates correctly.
use crate::entities::flag_capture;
use crate::entities::flag_capture::Model as FlagCaptureModel;
use crate::routes::admin::flag_captures::FlagCaptureListResponse;

#[utoipa::path(
    responses(
        (status = 200, description = "Flag captures for team successfully fetched.", body = Vec<FlagCaptureModel>),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_list_flag_captures_for_team",
    security(
        ("access_token" = [])
    ),
    tag = "admin/flag_captures"
)]
#[get("/list_by_team_id/{team_id}")]
pub async fn list_by_team_id(
    app_state: Data<AppState>,
    team_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let flag_captures = flag_capture::Entity::find()
        .filter(flag_capture::Column::Team.eq(team_id.into_inner()))
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(FlagCaptureListResponse { flag_captures }))
}
