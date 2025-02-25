use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(TeamStatus::Enum)
                    .values([TeamStatus::Confirmed, TeamStatus::Absent])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .add_column(
                        ColumnDef::new(Teams::ConfirmationCode)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .add_column(
                        ColumnDef::new(Teams::Status)
                            .custom(TeamStatus::Enum)
                            .not_null()
                            .extra("DEFAULT 'absent'"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .drop_column(Teams::ConfirmationCode)
                    .drop_column(Teams::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(TeamStatus::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    ConfirmationCode,
    Status,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "team_status")]
enum TeamStatus {
    #[sea_orm(iden = "team_status")]
    Enum,
    #[sea_orm(iden = "confirmed")]
    Confirmed,
    #[sea_orm(iden = "absent")]
    Absent,
}
