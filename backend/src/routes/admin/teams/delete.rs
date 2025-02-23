use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{delete, HttpResponse};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Team successfully deleted."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/teams"
)]
#[delete("/delete/{id}")]
pub async fn delete(
    app_state: Data<app_state::AppState>,
    id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    teams::Model::delete_as_admin(&app_state.database, id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}
