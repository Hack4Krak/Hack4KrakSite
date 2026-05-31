use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .drop_column(Teams::ConfirmationCode)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .add_column(
                        ColumnDef::new(Teams::ConfirmationCode)
                            .uuid()
                            .null()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    ConfirmationCode,
}
