use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users", rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(column_type = "String(StringLen::None)", unique)]
    pub username: String,
    #[sea_orm(column_type = "String(StringLen::None)", primary_key, unique)]
    pub email: String,
    #[sea_orm(column_type = "DateTime", default = "DateTime::now()")]
    pub created_at: DateTime,
    #[sea_orm(column_type = "String(StringLen::None)", nullable)]
    pub team_name: Option<String>,
    #[sea_orm(column_type = "String(StringLen::None)", nullable, unique)]
    pub leads: Option<String>,
    #[sea_orm(nullable)]
    pub permissions: Option<Vec<String>>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Teams,
    Leads,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Teams => Entity::belongs_to(super::teams::Entity)
                .from(Column::TeamName)
                .to(super::teams::Column::Name)
                .into(),
            Self::Leads => Entity::has_one(super::teams::Entity)
                .from(Column::Leads)
                .to(super::teams::Column::LeaderName)
                .into(),
        }
    }
}

impl Related<super::teams::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def();
        Relation::Leads.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
