pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250111_153247_intial::Migration),
            Box::new(m20250113_202045_auth_improvements::Migration),
        ]
    }
}
mod m20250111_153247_intial;
mod m20250113_202045_auth_improvements;
