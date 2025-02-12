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
                    .as_enum(UserRoles::Enum)
                    .values([UserRoles::Owner, UserRoles::Admin, UserRoles::Default])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::Roles)
                            .custom(UserRoles::Enum)
                            .not_null()
                            .extra("DEFAULT 'default'"),
                    )
                    .drop_column(Users::Permissions)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Roles)
                    .add_column(
                        ColumnDef::new(Users::Permissions)
                            .array(ColumnType::String(StringLen::None))
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(UserRoles::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Roles,
    Permissions,
}

#[derive(DeriveIden)]
#[sea_orm(enum_name = "user_roles")]
pub enum UserRoles {
    #[sea_orm(iden = "user_roles")]
    Enum,
    #[sea_orm(iden = "owner")]
    Owner,
    #[sea_orm(iden = "admin")]
    Admin,
    #[sea_orm(iden = "default")]
    Default,
}
