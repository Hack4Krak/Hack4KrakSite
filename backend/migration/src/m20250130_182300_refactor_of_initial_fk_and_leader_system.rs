use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE teams DROP CONSTRAINT fk_teams_leader_name;")
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Leads)
                    .add_column(
                        ColumnDef::new(Users::IsLeader)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .drop_column(Teams::LeaderName)
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared(
            "ALTER TABLE teams ADD CONSTRAINT teams_name_unique UNIQUE (name);\
            ALTER TABLE users ADD CONSTRAINT fk_users_team_name FOREIGN KEY (team_name) REFERENCES teams(name) ON DELETE SET NULL ON UPDATE SET NULL;"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE users DROP CONSTRAINT IF EXISTS fk_users_team_name;\
            ALTER TABLE teams DROP CONSTRAINT IF EXISTS teams_name_unique;",
        )
        .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::IsLeader)
                    .add_column(ColumnDef::new(Users::Leads).string().unique_key().null())
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .add_column(ColumnDef::new(Teams::LeaderName).unique_key().string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_teams_leader_name")
                    .from(Teams::Table, Teams::LeaderName)
                    .to(Users::Table, Users::Leads)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Leads,
    IsLeader,
}

#[derive(DeriveIden)]
enum Teams {
    Table,
    LeaderName,
}
