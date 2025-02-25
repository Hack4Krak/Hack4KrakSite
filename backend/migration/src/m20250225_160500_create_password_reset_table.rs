use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PasswordReset::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PasswordReset::Code).uuid().primary_key())
                    .col(
                        ColumnDef::new(PasswordReset::ExpirationDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PasswordReset::User).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("password_reset_user")
                            .from(PasswordReset::Table, PasswordReset::User)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PasswordReset::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum PasswordReset {
    Table,
    Code,
    ExpirationDate,
    User,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
