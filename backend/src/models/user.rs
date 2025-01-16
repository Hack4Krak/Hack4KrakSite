use crate::utils::error::Error;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, TransactionTrait};

use crate::models::entities::users;
use migration::Condition;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};

impl users::Model {
    pub async fn create_with_password(
        database: &DatabaseConnection,
        password_hash: String,
        register_json: &crate::routes::auth::RegisterModel,
    ) -> Result<(), Error> {
        let transaction = database.begin().await?;

        println!("a");
        if users::Entity::find()
            .filter(
                users::Column::Email
                    .eq(&register_json.email)
                    .or(users::Column::Username.eq(&register_json.name)),
            )
            .one(&transaction)
            .await?
            .is_some()
        {
            return Err(Error::UserAlreadyExists {});
        }
        println!("b");

        users::ActiveModel {
            username: Set(register_json.name.clone()),
            email: Set(register_json.email.clone()),
            password: Set(Some(password_hash)),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;
        println!("c");

        transaction.commit().await?;

        Ok(())
    }

    pub async fn verify_credentials(
        database: &DatabaseConnection,
        login_json: &crate::routes::auth::LoginModel,
    ) -> Result<Option<String>, Error> {
        let user_data = users::Entity::find()
            .filter(Condition::all().add(users::Column::Email.eq(&login_json.email)))
            .one(database)
            .await?
            .unwrap();

        let password = user_data.password.unwrap();
        let parsed_hash = PasswordHash::new(&password).unwrap();

        let is_verified = Argon2::default()
            .verify_password(login_json.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !is_verified {
            return Ok(None);
        }

        Ok(Some(user_data.email))
    }
}
