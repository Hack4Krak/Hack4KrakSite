//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.6

use super::sea_orm_active_enums::UserRoles;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    hack4krak_macros :: DeriveUpdatableModel,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    pub username: String,
    pub email: String,
    pub created_at: DateTime,
    pub password: Option<String>,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub is_leader: bool,
    pub team: Option<Uuid>,
    pub roles: UserRoles,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::password_reset::Entity")]
    PasswordReset,
    #[sea_orm(has_many = "super::team_invites::Entity")]
    TeamInvites,
    #[sea_orm(
        belongs_to = "super::teams::Entity",
        from = "Column::Team",
        to = "super::teams::Column::Id",
        on_update = "SetNull",
        on_delete = "SetNull"
    )]
    Teams,
}

impl Related<super::password_reset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PasswordReset.def()
    }
}

impl Related<super::team_invites::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamInvites.def()
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
