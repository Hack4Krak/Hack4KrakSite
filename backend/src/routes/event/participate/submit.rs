use super::{FoodPreferenceInput, ParticipationError};
use crate::entities::{event_registration, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::middlewares::event::EventMiddleware;
use crate::models::user::validate_name_chars;
use crate::services::identification::IdentificationService;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use actix_web_validation::Validated;
use chrono::Utc;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
struct ParticipateRequest {
    #[validate(length(min = 2, max = 256), custom(function = "validate_name_chars"))]
    pub full_name: String,
    #[validate(length(min = 2, max = 256), custom(function = "validate_name_chars"))]
    pub school: String,
    #[validate(length(min = 4, max = 4))]
    pub birth_year: String,
    #[validate(length(min = 5, max = 20))]
    pub phone: String,
    pub is_underage: bool,
    #[validate(length(max = 128), custom(function = "validate_name_chars"))]
    pub emergency_contact_name: Option<String>,
    #[validate(length(max = 20))]
    pub emergency_contact_phone: Option<String>,
    #[validate(email)]
    pub emergency_contact_email: Option<String>,
    pub food_preference: FoodPreferenceInput,
}

#[utoipa::path(
    request_body = ParticipateRequest,
    responses(
        (status = 200, description = "Registration submitted successfully."),
        (status = 400, description = "Registration period is not open."),
        (status = 409, description = "User is already registered."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
    tag = "event"
)]
#[post(
    "/participate",
    wrap = "AuthMiddleware::with_user()",
    wrap = "EventMiddleware::require_open_registration()"
)]
pub async fn submit_participation(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    Validated(body): Validated<Json<ParticipateRequest>>,
) -> Result<HttpResponse, Error> {
    let existing_registration = event_registration::Entity::find()
        .filter(event_registration::Column::UserId.eq(user.id))
        .one(&app_state.database)
        .await?;

    if existing_registration.is_some() {
        return Err(ParticipationError::AlreadyRegistered.into());
    }

    let body = body.into_inner();

    event_registration::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user.id),
        full_name: Set(body.full_name),
        school: Set(body.school),
        birth_year: Set(body.birth_year),
        phone: Set(body.phone),
        is_underage: Set(body.is_underage),
        emergency_contact_name: Set(body.emergency_contact_name),
        emergency_contact_phone: Set(body.emergency_contact_phone),
        emergency_contact_email: Set(body.emergency_contact_email),
        food_preference: Set(body.food_preference.into()),
        food_allergies: NotSet,
        registered_at: Set(Utc::now().naive_utc()),
    }
    .insert(&app_state.database)
    .await?;

    IdentificationService::send_identification_code_email(
        &app_state,
        &user.username,
        &user.email,
        user.identification_code,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
