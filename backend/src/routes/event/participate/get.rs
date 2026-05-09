use super::{FoodPreferenceInput, ParticipationError};
use crate::entities::{event_registration, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
struct ParticipateResponse {
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
    pub registered_at: chrono::NaiveDateTime,
}

impl From<event_registration::Model> for ParticipateResponse {
    fn from(registration: event_registration::Model) -> Self {
        Self {
            id: registration.id,
            user_id: registration.user_id,
            full_name: registration.full_name,
            school: registration.school,
            birth_year: registration.birth_year,
            phone: registration.phone,
            is_underage: registration.is_underage,
            emergency_contact_name: registration.emergency_contact_name,
            emergency_contact_phone: registration.emergency_contact_phone,
            emergency_contact_email: registration.emergency_contact_email,
            food_preference: registration.food_preference.into(),
            registered_at: registration.registered_at,
        }
    }
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
        .ok_or(ParticipationError::NotRegistered)?;

    Ok(HttpResponse::Ok().json(ParticipateResponse::from(registration)))
}
