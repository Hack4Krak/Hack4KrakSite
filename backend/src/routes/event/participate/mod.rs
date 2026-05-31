use crate::entities::sea_orm_active_enums::FoodPreference;
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::error::error_response_builder;
use actix_http::StatusCode;
use actix_web::error;
use hack4krak_macros::error_with_messages;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_actix_web::scope;

mod delete;
mod get;
mod submit;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(
        scope("")
            .wrap(AuthMiddleware::with_user())
            .service(submit::submit_participation)
            .service(get::get_participation)
            .service(delete::delete_participation),
    );
}

#[error_with_messages]
pub enum ParticipationError {
    AlreadyRegistered,
    NotRegistered,
    RegistrationNotOpen,
    StillInTeam,
}

impl error::ResponseError for ParticipationError {
    fn status_code(&self) -> StatusCode {
        match self {
            ParticipationError::RegistrationNotOpen => StatusCode::BAD_REQUEST,
            ParticipationError::NotRegistered => StatusCode::NOT_FOUND,
            ParticipationError::AlreadyRegistered | ParticipationError::StillInTeam => {
                StatusCode::CONFLICT
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum FoodPreferenceInput {
    Standard,
    Vegetarian,
}

impl From<FoodPreferenceInput> for FoodPreference {
    fn from(food_preference: FoodPreferenceInput) -> Self {
        match food_preference {
            FoodPreferenceInput::Standard => FoodPreference::Standard,
            FoodPreferenceInput::Vegetarian => FoodPreference::Vegetarian,
        }
    }
}

impl From<FoodPreference> for FoodPreferenceInput {
    fn from(food_preference: FoodPreference) -> Self {
        match food_preference {
            FoodPreference::Standard => FoodPreferenceInput::Standard,
            FoodPreference::Vegetarian => FoodPreferenceInput::Vegetarian,
        }
    }
}
