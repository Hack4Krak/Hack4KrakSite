pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250111_153247_intial::Migration),
            Box::new(m20250113_202045_auth_improvements::Migration),
            Box::new(m20250129_162814_add_team_invitation_field_to_user::Migration),
            Box::new(m20250130_145618_change_teams_and_users_pk_to_id_add_team_invites_table_drop_teaminvitations_column::Migration),
        ]
    }
}
mod m20250111_153247_intial;
mod m20250113_202045_auth_improvements;
mod m20250129_162814_add_team_invitation_field_to_user;
mod m20250130_145618_change_teams_and_users_pk_to_id_add_team_invites_table_drop_teaminvitations_column;
