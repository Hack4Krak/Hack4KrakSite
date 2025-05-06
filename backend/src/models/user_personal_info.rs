use crate::entities::user_personal_info;
use crate::utils::error::Error;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

impl user_personal_info::Model {
    pub async fn get_user_personal_information(
        user_id: Uuid,
        db: &sea_orm::DatabaseConnection,
    ) -> Result<Option<user_personal_info::Model>, Error> {
        Ok(user_personal_info::Entity::find()
            .filter(user_personal_info::Column::UserId.eq(user_id))
            .one(db)
            .await?)
    }
}
