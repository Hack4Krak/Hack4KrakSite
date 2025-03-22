use crate::entities::password_reset;
use crate::utils::error::Error;
use sea_orm::ActiveModelTrait;
use uuid::Uuid;

impl password_reset::Model {
    pub async fn create(
        database: &sea_orm::DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Uuid, Error> {
        let reset_code = Uuid::new_v4();

        let reset_model = password_reset::ActiveModel {
            code: sea_orm::ActiveValue::Set(reset_code),
            expiration_date: sea_orm::ActiveValue::Set(
                chrono::Local::now().naive_local() + chrono::Duration::hours(1),
            ),
            user: sea_orm::ActiveValue::Set(user_id),
        };

        reset_model.insert(database).await?;

        Ok(reset_code)
    }
}
