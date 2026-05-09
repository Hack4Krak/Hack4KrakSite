use super::ParticipationError;
use crate::entities::{event_registration, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Data;
use actix_web::{HttpResponse, delete};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[utoipa::path(
    responses(
        (status = 200, description = "Registration cancelled successfully."),
        (status = 409, description = "User must leave or delete their team first."),
        (status = 404, description = "User has not registered yet."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
    tag = "event"
)]
#[delete("/participate", wrap = "AuthMiddleware::with_user()")]
pub async fn delete_participation(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    if user.team.is_some() {
        return Err(ParticipationError::StillInTeam.into());
    }

    let result = event_registration::Entity::delete_many()
        .filter(event_registration::Column::UserId.eq(user.id))
        .exec(&app_state.database)
        .await?;

    if result.rows_affected == 0 {
        return Err(ParticipationError::NotRegistered.into());
    }

    Ok(SuccessResponse::default().http_response())
}
