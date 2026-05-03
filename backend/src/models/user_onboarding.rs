use crate::entities::user_onboarding::Model;
use crate::entities::{user_onboarding, users};
use crate::routes::account::AccountError;
use crate::routes::account::UserOnboardingSubmissionRequest;
use crate::utils::error::Error;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use serde_json::to_value;

pub const ALLOWED_REFERRAL_SOURCES: [&str; 8] = [
    "Instagram",
    "Linkedin",
    "Facebook",
    "Discord",
    "Friend",
    "School",
    "Internet",
    "Other",
];

impl Model {
    pub async fn create(
        database: &DatabaseConnection,
        user: users::Model,
        request_body: UserOnboardingSubmissionRequest,
    ) -> Result<(), Error> {
        if user.onboarding.is_some() {
            return Err(AccountError::OnboardingAlreadySubmitted.into());
        }

        let onboarding_id = uuid::Uuid::new_v4();

        let transaction = database.begin().await?;

        user_onboarding::Entity::insert(user_onboarding::ActiveModel {
            id: Set(onboarding_id),
            organization: Set(request_body.organization),
            location: Set(request_body.location),
            ctf_experience: Set(request_body.ctf_experience),
            school_grade: Set(request_body.school_grade),
            collab_interest: Set(request_body.collab_interest),
            marketing_consent: Set(request_body.marketing_consent),
            marketing_consent_accepted_at: Set(chrono::Utc::now().naive_utc()),
            marketing_consent_updated_at: Set(chrono::Utc::now().naive_utc()),
            referral_sources: Set(Some(to_value(request_body.referral_sources)?)),
        })
        .exec(&transaction)
        .await?;

        let updatable_user = users::UpdatableModel {
            onboarding: Some(Some(onboarding_id)),
            ..Default::default()
        };

        let active_user = updatable_user.update(user);
        active_user.save(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}
