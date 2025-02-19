use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Teams successfully fetched."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/teams"
)]
#[get("/")]
pub async fn teams_list(app_state: Data<app_state::AppState>) -> Result<HttpResponse, Error> {
    let teams = teams::Model::list(&app_state.database).await?;

    Ok(HttpResponse::Ok().json(teams))
}
