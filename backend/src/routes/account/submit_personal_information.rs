use crate::entities::{user_personal_info, users};
use crate::models::user_personal_info::{ALLOWED_REFERRAL_SOURCES, MAX_AGE};
use crate::routes::account::AccountError::{BirthYearNotInRange, InvalidReferralSource};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use chrono::Datelike;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserPersonalInformationSubmissionRequest {
    pub first_name: String,
    pub birth_year: i32,
    pub location: String,
    pub organization: String,
    pub is_vegetarian: bool,
    pub marketing_consent: bool,
    pub referral_source: Option<Vec<String>>,
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
    let current_year = chrono::Utc::now().year();

    if request_body.birth_year > current_year || request_body.birth_year < current_year - 120 {
        return Err(Error::Account(BirthYearNotInRange {
            min: current_year - MAX_AGE,
            max: current_year,
        }));
    }

    if let Some(referral_source) = &request_body.referral_source
        && referral_source.iter().any(|source| {
            !ALLOWED_REFERRAL_SOURCES
                .iter()
                .any(|allowed_source| allowed_source == source)
        })
    {
        return Err(Error::Account(InvalidReferralSource));
    }

    user_personal_info::Model::create(&app_state.database, user, request_body).await?;

    Ok(SuccessResponse::default().http_response())
}
