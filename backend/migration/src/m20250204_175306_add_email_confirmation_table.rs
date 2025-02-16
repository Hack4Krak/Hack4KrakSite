use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EmailConfirmation::Table)
                    .col(ColumnDef::new(EmailConfirmation::Email).string().not_null())
                    .col(
                        ColumnDef::new(EmailConfirmation::UserInfo)
                            .json()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailConfirmation::Code)
                            .string()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailConfirmation::ExpirationDate)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EmailConfirmation::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum EmailConfirmation {
    Table,
    Email,
    UserInfo,
    Code,
    ExpirationDate,
}
