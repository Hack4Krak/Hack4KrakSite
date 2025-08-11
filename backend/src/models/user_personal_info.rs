use crate::entities::user_personal_info::{Column, Model};
use crate::entities::{user_personal_info, users};
use crate::routes::account::UserPersonalInformationSubmissionRequest;
use crate::utils::error::Error;
use migration::OnConflict;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TransactionTrait};
use serde_json::to_value;

pub const ALLOWED_REFERRAL_SOURCES: [&str; 6] = [
    "Linkedin",
    "Facebook",
    "Instagram",
    "Znajomy",
    "Internet",
    "Inne",
];

pub const MAX_AGE: i32 = 120;

impl Model {
    pub async fn create(
        database: &DatabaseConnection,
        user: users::Model,
        request_body: UserPersonalInformationSubmissionRequest,
    ) -> Result<(), Error> {
        let personal_info_id = user.personal_info.unwrap_or(uuid::Uuid::new_v4());

        let transaction = database.begin().await?;

        user_personal_info::Entity::insert(user_personal_info::ActiveModel {
            id: Set(personal_info_id),
            first_name: Set(request_body.first_name),
            birth_year: Set(request_body.birth_year),
            location: Set(request_body.location),
            organization: Set(request_body.organization),
            is_vegetarian: Set(request_body.is_vegetarian),
            marketing_consent: Set(request_body.marketing_consent),
            marketing_consent_accepted_at: Set(chrono::Utc::now().naive_utc()),
            marketing_consent_updated_at: Set(chrono::Utc::now().naive_utc()),
            referral_source: Set(request_body.referral_source.map(to_value).transpose()?),
        })
        .on_conflict(
            OnConflict::columns(vec![Column::Id])
                .update_columns(vec![
                    Column::FirstName,
                    Column::BirthYear,
                    Column::Location,
                    Column::Organization,
                    Column::IsVegetarian,
                    Column::MarketingConsent,
                    Column::MarketingConsentAcceptedAt,
                    Column::MarketingConsentUpdatedAt,
                    Column::ReferralSource,
                ])
                .to_owned(),
        )
        .exec(&transaction)
        .await?;

        if user.personal_info.is_none() {
            let updatable_user = users::UpdatableModel {
                personal_info: Some(Some(personal_info_id)),
                ..Default::default()
            };

            let active_user = updatable_user.update(user);
            active_user.save(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(())
    }
}
