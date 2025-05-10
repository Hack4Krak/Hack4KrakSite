use crate::AuthMiddleware;
use crate::entities::{user_personal_info, users};
use crate::models::user_personal_info::ALLOWED_REFERRAL_SOURCES;
use crate::routes::account::AccountError::{BirthYearNotInRange, InvalidReferralSource};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use chrono::Datelike;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct UserPersonalInformationSubmissionRequest {
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
        (status = 200, description = "User information received."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[post("/submit_personal_information", wrap = "AuthMiddleware::with_user()")]
pub async fn submit_personal_information(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    request_body: Json<UserPersonalInformationSubmissionRequest>,
) -> Result<HttpResponse, Error> {
    let request_body = request_body.into_inner();
    let current_year = chrono::Utc::now().year();

    if request_body.birth_year > current_year || request_body.birth_year < current_year - 120 {
        return Err(Error::Account(BirthYearNotInRange {
            min: current_year - 120,
            max: current_year,
        }));
    }

    if let Some(referral_source) = &request_body.referral_source {
        for source in referral_source {
            if !ALLOWED_REFERRAL_SOURCES.contains(&source.as_str()) {
                return Err(Error::Account(InvalidReferralSource));
            }
        }
    }

    if let Some(personal_info_id) = user.personal_info {
        let personal_info = user_personal_info::Entity::find_by_id(personal_info_id)
            .one(&app_state.database)
            .await?
            .ok_or(Error::Unauthorized)?;

        let personal_info_to_update = user_personal_info::UpdatableModel {
            first_name: Some(request_body.first_name),
            birth_year: Some(request_body.birth_year),
            location: Some(request_body.location),
            organization: Some(request_body.organization),
            is_vegetarian: Some(request_body.is_vegetarian),
            marketing_consent: Some(request_body.marketing_consent),
            marketing_consent_accepted_at: Some(chrono::Utc::now().naive_utc()),
            referral_source: Some(request_body.referral_source.map(to_value).transpose()?),
        };

        let active_personal_info = personal_info_to_update.update(personal_info);

        active_personal_info.save(&app_state.database).await?;
    } else {
        let transaction = app_state.database.begin().await?;

        let id = uuid::Uuid::new_v4();

        user_personal_info::ActiveModel {
            id: Set(id),
            first_name: Set(request_body.first_name),
            birth_year: Set(request_body.birth_year),
            location: Set(request_body.location),
            organization: Set(request_body.organization),
            is_vegetarian: Set(request_body.is_vegetarian),
            marketing_consent: Set(request_body.marketing_consent),
            marketing_consent_accepted_at: Set(chrono::Utc::now().naive_utc()),
            referral_source: Set(request_body.referral_source.map(to_value).transpose()?),
        }
        .insert(&transaction)
        .await?;

        let updatable_user = users::UpdatableModel {
            personal_info: Some(Some(id)),
            ..Default::default()
        };

        let active_user = updatable_user.update(user);

        active_user.save(&transaction).await?;

        transaction.commit().await?;
    }

    Ok(SuccessResponse::default().http_response())
}
