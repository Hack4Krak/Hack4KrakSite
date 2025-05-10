use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{boolean, integer, pk_uuid, string, timestamp, uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPersonalInfo::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserPersonalInfo::Id))
                    .col(uuid(UserPersonalInfo::UserId).unique_key())
                    .col(string(UserPersonalInfo::FirstName))
                    .col(integer(UserPersonalInfo::BirthYear))
                    .col(string(UserPersonalInfo::Location))
                    .col(string(UserPersonalInfo::Organization))
                    .col(boolean(UserPersonalInfo::IsVegetarian).default(false))
                    .col(boolean(UserPersonalInfo::MarketingConsent).default(false))
                    .col(timestamp(UserPersonalInfo::MarketingConsentAcceptedAt))
                    .col(ColumnDef::new(UserPersonalInfo::ReferralSource).json())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_personal_info-user_id")
                            .from(UserPersonalInfo::Table, UserPersonalInfo::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPersonalInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserPersonalInfo {
    Table,
    Id,
    UserId,
    FirstName,
    BirthYear,
    Location,
    Organization,
    IsVegetarian,
    MarketingConsent,
    MarketingConsentAcceptedAt,
    ReferralSource,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
