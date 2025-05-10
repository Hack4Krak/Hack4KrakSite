use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::PersonalInfo).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-user_personal_info-user_id")
                    .table(UserPersonalInfo::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserPersonalInfo::Table)
                    .drop_column(UserPersonalInfo::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_personal_info-user_personal_info_id")
                    .from(Users::Table, Users::PersonalInfo)
                    .to(UserPersonalInfo::Table, UserPersonalInfo::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-user_personal_info-user_personal_info_id")
                    .table(Users::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserPersonalInfo::Table)
                    .add_column(ColumnDef::new(UserPersonalInfo::UserId).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_personal_info-user_id")
                    .from(UserPersonalInfo::Table, UserPersonalInfo::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::PersonalInfo)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserPersonalInfo {
    Table,
    Id,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    PersonalInfo,
}
