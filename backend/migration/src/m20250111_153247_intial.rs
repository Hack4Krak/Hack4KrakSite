use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::UserName)
                            .not_null()
                            .string()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .not_null()
                            .string()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .not_null()
                            .date_time()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(ColumnDef::new(Users::TeamName).string().null())
                    .col(
                        ColumnDef::new(Users::Permissions)
                            .array(ColumnType::String(StringLen::None))
                            .null(),
                    )
                    .col(ColumnDef::new(Users::Leads).string().unique_key().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Teams::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Teams::Name)
                            .not_null()
                            .string()
                            .unique_key()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Teams::CreatedAt)
                            .not_null()
                            .date_time()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(Teams::LeaderName)
                            .not_null()
                            .unique_key()
                            .string(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_users_team_name")
                    .from(Users::Table, Users::TeamName)
                    .to(Teams::Table, Teams::Name)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_users_leads")
                    .from(Users::Table, Users::Leads)
                    .to(Teams::Table, Teams::Name)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_teams_leader_name")
                    .from(Teams::Table, Teams::LeaderName)
                    .to(Users::Table, Users::UserName)
                    .on_delete(ForeignKeyAction::SetDefault)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    UserName,
    Email,
    CreatedAt,
    TeamName,
    Leads,
    Permissions,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    Name,
    CreatedAt,
    LeaderName,
}
