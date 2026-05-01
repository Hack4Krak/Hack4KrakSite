use crate::entities::{user_personal_info, users};
use crate::entities::sea_orm_active_enums::{CtfExperience, SchoolGrade};
use crate::routes::account::AccountError::InvalidReferralSource;
use crate::models::user_personal_info::ALLOWED_REFERRAL_SOURCES;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserPersonalInformationSubmissionRequest {
    pub first_name: String,
    pub location: String,
    pub ctf_experience: CtfExperience,
    pub school_grade: SchoolGrade,
    pub referral_sources: Vec<String>,
    pub marketing_consent: bool,
    pub collab_interest: bool,
}

#[utoipa::path(
    request_body = UserPersonalInformationSubmissionRequest,
    responses(
        (status = 200, description = "User personal information submitted."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[post("/submit_personal_information")]
pub async fn submit_personal_information(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    request_body: Json<UserPersonalInformationSubmissionRequest>,
) -> Result<HttpResponse, Error> {
    let request_body = request_body.into_inner();

    if request_body
        .referral_sources
        .iter()
        .any(|source| !ALLOWED_REFERRAL_SOURCES.iter().any(|allowed| allowed == source))
    {
        return Err(Error::Account(InvalidReferralSource));
    }

    user_personal_info::Model::create(&app_state.database, user, request_body).await?;

    Ok(SuccessResponse::default().http_response())
}
