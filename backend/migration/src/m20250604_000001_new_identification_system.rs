use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{array, pk_uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::VerificationId)
                            .uuid()
                            .unique_key()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserParticipantTags::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserParticipantTags::UserId))
                    .col(
                        array(
                            UserParticipantTags::Tags,
                            ColumnType::String(Default::default()),
                        )
                        .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_participant_tags-user_id")
                    .from(UserParticipantTags::Table, UserParticipantTags::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-user_participant_tags-user_id")
                    .table(UserParticipantTags::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserParticipantTags::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::VerificationId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    VerificationId,
}

#[derive(DeriveIden)]
enum UserParticipantTags {
    Table,
    UserId,
    Tags,
}
