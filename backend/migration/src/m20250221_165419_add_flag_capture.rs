use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FlagCapture::Table)
                    .if_not_exists()
                    .col(pk_auto(FlagCapture::Id))
                    .col(uuid(FlagCapture::Team).not_null())
                    .col(string(FlagCapture::Task).not_null())
                    .col(
                        ColumnDef::new(FlagCapture::SubmittedAt)
                            .not_null()
                            .date_time()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(FlagCapture::Table, FlagCapture::Team)
                            .to(Teams::Table, Teams::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("unique_team_task")
                            .unique()
                            .col(FlagCapture::Team)
                            .col(FlagCapture::Task),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FlagCapture::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FlagCapture {
    Table,
    Id,
    Team,
    Task,
    SubmittedAt,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
}
