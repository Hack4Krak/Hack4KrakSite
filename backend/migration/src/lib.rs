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
            Box::new(m20250130_182300_refactor_of_initial_fk_and_leader_system::Migration),
            Box::new(m20250131_155122_rename_teamname_to_team_and_change_relation_to_team_id_in_users::Migration),
            Box::new(m20250203_105134_setup_roles_system::Migration),
        ]
    }
}
mod m20250111_153247_intial;
mod m20250113_202045_auth_improvements;
mod m20250129_162814_add_team_invitation_field_to_user;
mod m20250130_145618_change_teams_and_users_pk_to_id_add_team_invites_table_drop_teaminvitations_column;
mod m20250130_182300_refactor_of_initial_fk_and_leader_system;
mod m20250131_155122_rename_teamname_to_team_and_change_relation_to_team_id_in_users;
mod m20250203_105134_setup_roles_system;
