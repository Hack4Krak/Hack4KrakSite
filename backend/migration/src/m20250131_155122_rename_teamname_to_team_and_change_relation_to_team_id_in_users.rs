use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE users DROP CONSTRAINT IF EXISTS fk_users_team_name;")
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::TeamName)
                    .add_column(ColumnDef::new(Users::Team).uuid().null())
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared(
            "ALTER TABLE users ADD CONSTRAINT fk_users_team_id FOREIGN KEY (team) REFERENCES teams(id) ON DELETE SET NULL ON UPDATE SET NULL;"
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE users DROP CONSTRAINT IF EXISTS fk_users_team_id;")
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Team)
                    .add_column(ColumnDef::new(Users::TeamName).string().null())
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared(
            "ALTER TABLE users ADD CONSTRAINT fk_users_team_name FOREIGN KEY (team_name) REFERENCES teams(name) ON DELETE SET NULL ON UPDATE SET NULL;"
        ).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    TeamName,
    Team,
}
