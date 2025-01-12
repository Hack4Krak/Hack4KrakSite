use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(column_type = "String(StringLen::None)", primary_key, unique)]
    pub name: String,
    #[sea_orm(column_type = "DateTime", default = "DateTime::now()")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "String(StringLen::None)", default = "Not set")]
    pub leader_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Leader,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::has_many(super::users::Entity).into(),
            Self::Leader => Entity::belongs_to(super::users::Entity)
                .from(Column::LeaderName)
                .to(super::users::Column::Username)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def();
        Relation::Leader.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
