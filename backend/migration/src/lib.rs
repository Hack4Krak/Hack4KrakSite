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
            Box::new(m20250204_175306_add_email_confirmation_table::Migration),
            Box::new(m20250221_165419_add_flag_capture::Migration),
            Box::new(m20250225_160500_create_password_reset_table::Migration),
            Box::new(m20250225_171450_add_confirmation_code_and_status_fields_to_team::Migration),
            Box::new(m20250429_123733_user_personal_info::Migration),
            Box::new(m20250430_183841_unified_email_verification::Migration),
            Box::new(m20250503_182326_create_external_team_invitation::Migration),
            Box::new(m20250526_105526_add_team_color_field::Migration),
            Box::new(m20250526_172108_add_organization_column_to_team::Migration),
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
mod m20250204_175306_add_email_confirmation_table;
mod m20250221_165419_add_flag_capture;
mod m20250225_160500_create_password_reset_table;
mod m20250225_171450_add_confirmation_code_and_status_fields_to_team;
mod m20250429_123733_user_personal_info;
mod m20250430_183841_unified_email_verification;
mod m20250503_182326_create_external_team_invitation;
mod m20250526_105526_add_team_color_field;
mod m20250526_172108_add_organization_column_to_team;
