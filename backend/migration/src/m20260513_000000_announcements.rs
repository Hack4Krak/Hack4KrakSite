use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{date_time, json_binary_null, pk_uuid, string};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Announcement::Table)
                    .if_not_exists()
                    .col(pk_uuid(Announcement::Id))
                    .col(string(Announcement::ActionType))
                    .col(json_binary_null(Announcement::AdditionalData))
                    .col(date_time(Announcement::CreatedAt).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_announcement_action_type_created_at")
                    .table(Announcement::Table)
                    .col(Announcement::ActionType)
                    .col(Announcement::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_announcement_action_type_created_at")
                    .table(Announcement::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Announcement::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Announcement {
    Table,
    Id,
    ActionType,
    AdditionalData,
    CreatedAt,
}
