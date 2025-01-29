use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE users DROP CONSTRAINT IF EXISTS fk_users_team_name;\
            ALTER TABLE users DROP CONSTRAINT IF EXISTS fk_users_leads;\
            ALTER TABLE users DROP CONSTRAINT IF EXISTS users_pkey; \
             ALTER TABLE teams DROP CONSTRAINT IF EXISTS teams_pkey;",
        )
        .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::TeamInvitations)
                    .add_column(
                        ColumnDef::new(Users::Id)
                            .not_null()
                            .uuid()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .add_column(
                        ColumnDef::new(Teams::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TeamInvites::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TeamInvites::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TeamInvites::User).uuid().not_null())
                    .col(ColumnDef::new(TeamInvites::Team).big_integer().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_team_invites_user")
                    .from(TeamInvites::Table, TeamInvites::User)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_team_invites_team")
                    .from(TeamInvites::Table, TeamInvites::Team)
                    .to(Teams::Table, Teams::Id)
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
                    .name("fk_team_invites_user")
                    .table(TeamInvites::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_team_invites_team")
                    .table(TeamInvites::Table)
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();
        db.execute_unprepared(
            "ALTER TABLE users DROP CONSTRAINT IF EXISTS users_pkey; \
             ALTER TABLE teams DROP CONSTRAINT IF EXISTS teams_pkey;",
        )
        .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Users::TeamInvitations)
                            .array(ColumnType::String(StringLen::None))
                            .null(),
                    )
                    .drop_column(Users::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .drop_column(Teams::Id)
                    .to_owned(),
            )
            .await?;
        db.execute_unprepared(
            "ALTER TABLE users ADD CONSTRAINT users_pkey PRIMARY KEY (email);\
            ALTER TABLE teams ADD CONSTRAINT teams_pkey PRIMARY KEY (name);",
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
            .drop_table(Table::drop().table(TeamInvites::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    TeamInvitations,
    TeamName,
    Leads,
}
#[derive(DeriveIden)]
enum Teams {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum TeamInvites {
    Table,
    Id,
    User,
    Team,
}
