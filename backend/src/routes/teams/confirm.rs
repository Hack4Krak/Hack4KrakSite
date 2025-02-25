use crate::entities::teams;
use crate::routes::teams::TeamError::InvalidConfirmationCode;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, post};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully confirmed team."),
        (status = 404, description = "Invalid confirmation code"),
        (status = 500, description = "Internal server error.")
    ),
    tag = "teams"
)]
#[post("/confirm/{confirmation_code}")]
pub async fn confirm(
    app_state: Data<app_state::AppState>,
    confirmation_code: Path<String>,
) -> Result<HttpResponse, Error> {
    let confirmation_code =
        Uuid::parse_str(&confirmation_code).map_err(|_| Error::Team(InvalidConfirmationCode))?;
    teams::Model::confirm(&app_state.database, confirmation_code).await?;

    Ok(SuccessResponse::default().http_response())
}
