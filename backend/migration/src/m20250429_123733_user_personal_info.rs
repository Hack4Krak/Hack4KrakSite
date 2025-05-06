use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, integer, pk_uuid, string, timestamp};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::PersonalInfo).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserPersonalInfo::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserPersonalInfo::Id))
                    .col(string(UserPersonalInfo::FirstName))
                    .col(integer(UserPersonalInfo::BirthYear))
                    .col(string(UserPersonalInfo::Location))
                    .col(string(UserPersonalInfo::Organization))
                    .col(boolean(UserPersonalInfo::IsVegetarian))
                    .col(boolean(UserPersonalInfo::MarketingConsent))
                    .col(timestamp(UserPersonalInfo::MarketingConsentAcceptedAt))
                    .col(timestamp(UserPersonalInfo::MarketingConsentUpdatedAt))
                    .col(ColumnDef::new(UserPersonalInfo::ReferralSource).json())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_personal_info-user_personal_info_id")
                    .from(Users::Table, Users::PersonalInfo)
                    .to(UserPersonalInfo::Table, UserPersonalInfo::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-user_personal_info-user_personal_info_id")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::PersonalInfo)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserPersonalInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPersonalInfo {
    Table,
    Id,
    FirstName,
    BirthYear,
    Location,
    Organization,
    IsVegetarian,
    MarketingConsent,
    MarketingConsentAcceptedAt,
    MarketingConsentUpdatedAt,
    ReferralSource,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    PersonalInfo,
}
