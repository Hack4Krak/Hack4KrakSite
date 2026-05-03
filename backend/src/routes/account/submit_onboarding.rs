use crate::entities::sea_orm_active_enums::{CtfExperience, SchoolGrade};
use crate::entities::{user_onboarding, users};
use crate::models::user::validate_name_chars;
use crate::models::user_onboarding::ALLOWED_REFERRAL_SOURCES;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserOnboardingSubmissionRequest {
    #[validate(length(min = 3, max = 128), custom(function = "validate_name_chars"))]
    pub organization: String,
    #[validate(length(min = 2, max = 128), custom(function = "validate_name_chars"))]
    pub location: String,
    pub ctf_experience: CtfExperience,
    pub school_grade: SchoolGrade,
    #[validate(
        length(min = 1, max = 8),
        custom(function = "validate_referral_sources")
    )]
    pub referral_sources: Vec<String>,
    pub marketing_consent: bool,
    pub collab_interest: bool,
}

fn validate_referral_sources(referral_sources: &[String]) -> Result<(), ValidationError> {
    if referral_sources.iter().all(|source| {
        ALLOWED_REFERRAL_SOURCES
            .iter()
            .any(|allowed_source| allowed_source == source)
    }) {
        return Ok(());
    }

    Err(ValidationError::new("invalid_referral_source"))
}

#[utoipa::path(
    request_body = UserOnboardingSubmissionRequest,
    responses(
        (status = 200, description = "User onboarding answers submitted."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "submit_onboarding",
    tag = "account"
)]
#[post("/onboarding")]
pub async fn submit_onboarding(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    Validated(request_body): Validated<Json<UserOnboardingSubmissionRequest>>,
) -> Result<HttpResponse, Error> {
    user_onboarding::Model::create(&app_state.database, user, request_body.into_inner()).await?;

    Ok(SuccessResponse::default().http_response())
}
