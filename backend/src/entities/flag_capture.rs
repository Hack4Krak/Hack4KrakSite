//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "flag_capture")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub team: Uuid,
    pub task: String,
    pub submitted_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::teams::Entity",
        from = "Column::Team",
        to = "super::teams::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Teams,
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
