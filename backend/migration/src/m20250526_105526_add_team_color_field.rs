use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .add_column(string_len(Teams::Color, 7).default("#FFFFFF"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .drop_column(Teams::Color)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Color,
}
