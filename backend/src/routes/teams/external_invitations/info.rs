use crate::entities::email_verification_request;
use crate::models::email_verification_request::EmailVerificationAction;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use uuid::Uuid;

#[utoipa::path(
    description = "Create external team invitations using secret token",
    responses(
        (status = 200, body = String),
        (status = 400),
        (status = 500)
    ),
    operation_id = "external_invitations_info",
    security(
        ("access_token" = [])
    ),
    tag = "teams/external_invitations"
)]
#[get("/info/{code}")]
pub async fn info(
    app_state: Data<app_state::AppState>,
    confirmation_code: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let confirmation_code = confirmation_code.into_inner();

    let email_verification_request =
        email_verification_request::Model::find_and_verify(&app_state.database, confirmation_code)
            .await?;
    let EmailVerificationAction::RegisterTeam { organization } =
        email_verification_request.get_action()?
    else {
        return Err(Error::InvalidEmailConfirmationCode);
    };

    Ok(HttpResponse::Ok().body(organization))
}
