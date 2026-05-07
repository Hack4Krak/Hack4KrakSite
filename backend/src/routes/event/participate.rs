use crate::entities::sea_orm_active_enums::FoodPreference;
use crate::entities::{event_registration, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::models::user::validate_name_chars;
use crate::routes::event::EventError;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, delete, get, post};
use actix_web_validation::Validated;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FoodPreferenceInput {
    Standard,
    Vegetarian,
}

impl From<FoodPreferenceInput> for FoodPreference {
    fn from(val: FoodPreferenceInput) -> Self {
        match val {
            FoodPreferenceInput::Standard => FoodPreference::Standard,
            FoodPreferenceInput::Vegetarian => FoodPreference::Vegetarian,
        }
    }
}

impl From<FoodPreference> for FoodPreferenceInput {
    fn from(val: FoodPreference) -> Self {
        match val {
            FoodPreference::Standard => FoodPreferenceInput::Standard,
            FoodPreference::Vegetarian => FoodPreferenceInput::Vegetarian,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ParticipateRequest {
    #[validate(length(min = 2, max = 128), custom(function = "validate_name_chars"))]
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
    #[validate(length(max = 512))]
    pub food_allergies: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ParticipateResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub full_name: String,
    pub school: String,
    pub birth_year: String,
    pub phone: String,
    pub is_underage: bool,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub emergency_contact_email: Option<String>,
    pub food_preference: FoodPreferenceInput,
    pub food_allergies: Option<String>,
    pub registered_at: chrono::NaiveDateTime,
}

impl From<event_registration::Model> for ParticipateResponse {
    fn from(m: event_registration::Model) -> Self {
        Self {
            id: m.id,
            user_id: m.user_id,
            full_name: m.full_name,
            school: m.school,
            birth_year: m.birth_year,
            phone: m.phone,
            is_underage: m.is_underage,
            emergency_contact_name: m.emergency_contact_name,
            emergency_contact_phone: m.emergency_contact_phone,
            emergency_contact_email: m.emergency_contact_email,
            food_preference: m.food_preference.into(),
            food_allergies: m.food_allergies,
            registered_at: m.registered_at,
        }
    }
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
#[post("/participate", wrap = "AuthMiddleware::with_user()")]
pub async fn submit_participation(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    Validated(body): Validated<Json<ParticipateRequest>>,
) -> Result<HttpResponse, Error> {
    let registration_config = app_state.task_manager.registration_config.read().await;
    if !registration_config.is_open() {
        return Err(EventError::RegistrationNotOpen.into());
    }
    drop(registration_config);

    let existing = event_registration::Entity::find()
        .filter(event_registration::Column::UserId.eq(user.id))
        .one(&app_state.database)
        .await?;

    if existing.is_some() {
        return Err(EventError::AlreadyRegistered.into());
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
        food_allergies: Set(body.food_allergies),
        registered_at: Set(Utc::now().naive_utc()),
    }
    .insert(&app_state.database)
    .await?;

    Ok(SuccessResponse::default().http_response())
}

#[utoipa::path(
    responses(
        (status = 200, description = "Registration data returned.", body = ParticipateResponse),
        (status = 404, description = "User has not registered yet."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
    tag = "event"
)]
#[get("/participate", wrap = "AuthMiddleware::with_user()")]
pub async fn get_participation(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let registration = event_registration::Entity::find()
        .filter(event_registration::Column::UserId.eq(user.id))
        .one(&app_state.database)
        .await?
        .ok_or(EventError::NotRegistered)?;

    Ok(HttpResponse::Ok().json(ParticipateResponse::from(registration)))
}

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
        return Err(EventError::StillInTeam.into());
    }

    let result = event_registration::Entity::delete_many()
        .filter(event_registration::Column::UserId.eq(user.id))
        .exec(&app_state.database)
        .await?;

    if result.rows_affected == 0 {
        return Err(EventError::NotRegistered.into());
    }

    Ok(SuccessResponse::default().http_response())
}
