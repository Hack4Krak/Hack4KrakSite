use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{pk_uuid, string_len_uniq, uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExternalTeamInvitation::Table)
                    .if_not_exists()
                    .col(pk_uuid(ExternalTeamInvitation::Id))
                    .col(uuid(ExternalTeamInvitation::Team))
                    .col(uuid(ExternalTeamInvitation::AdministrationCode))
                    .col(string_len_uniq(ExternalTeamInvitation::AccessCode, 6))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_external_team_invitation_team")
                    .from(ExternalTeamInvitation::Table, ExternalTeamInvitation::Team)
                    .to(Teams::Table, Teams::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_external_team_invitation_administration_code")
                    .table(ExternalTeamInvitation::Table)
                    .col(ExternalTeamInvitation::AdministrationCode)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_external_team_invitation_team")
                    .table(ExternalTeamInvitation::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(ExternalTeamInvitation::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ExternalTeamInvitation {
    Table,
    Id,
    Team,
    AdministrationCode,
    AccessCode,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
}
