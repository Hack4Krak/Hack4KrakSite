use actix_web::HttpResponse;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use migration::Condition;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait, DatabaseConnection, TransactionTrait};

use crate::models::entities::users;
use crate::routes::auth::AuthError::{
    InvalidCredentials, PasswordAuthNotAvailable, UserAlreadyExists,
};
use crate::routes::auth::{LoginModel, RegisterModel};
use crate::utils::env::Config;
use crate::utils::error::Error;
use crate::utils::jwt::append_tokens_as_cookies;
use sea_orm::prelude::Uuid as SeaOrmUuid;
use uuid::Uuid as uuid_gen;

impl users::Model {
    pub async fn create_from_oauth(
        database: &DatabaseConnection,
        username: String,
        email: String,
    ) -> Result<HttpResponse, Error> {
        let transaction = database.begin().await?;

        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&email))
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        let uuid = if let Some(uuid) = user.team {
            uuid
        } else {
            let new_uuid = uuid_gen::new_v4();
            users::ActiveModel {
                id: Set(new_uuid),
                username: Set(username),
                email: Set(email.clone()),
                password: Set(None),
                ..Default::default()
            }
            .insert(&transaction)
            .await?;

            new_uuid
        };

        let mut response = HttpResponse::Ok();
        response.append_header((
            "Refresh",
            format!("0; {}", Config::get().oauth_finish_redirect_url.clone()),
        ));
        append_tokens_as_cookies(uuid, email, &mut response)?;
        Ok(response.body("Redirecting..."))
    }

    pub async fn create_with_password(
        database: &DatabaseConnection,
        uuid: SeaOrmUuid,
        password_hash: String,
        register_json: &RegisterModel,
    ) -> Result<(), Error> {
        let transaction = database.begin().await?;

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
            return Err(Error::Auth(UserAlreadyExists));
        }

        users::ActiveModel {
            id: Set(uuid),
            username: Set(register_json.name.clone()),
            email: Set(register_json.email.clone()),
            password: Set(Some(password_hash)),
            ..Default::default()
        }
        .insert(&transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn verify_credentials(
        database: &DatabaseConnection,
        login_json: &LoginModel,
    ) -> Result<(SeaOrmUuid, String), Error> {
        let user_data = users::Entity::find()
            .filter(Condition::all().add(users::Column::Email.eq(&login_json.email)))
            .one(database)
            .await?
            .ok_or(Error::Auth(InvalidCredentials))?;

        let password = user_data
            .password
            .ok_or(Error::Auth(PasswordAuthNotAvailable))?;
        let parsed_hash = PasswordHash::new(&password).map_err(Error::HashPasswordFailed)?;

        let is_verified = Argon2::default()
            .verify_password(login_json.password.as_bytes(), &parsed_hash)
            .is_ok();

        if !is_verified {
            return Err(Error::Auth(InvalidCredentials));
        }

        Ok((user_data.id, user_data.email))
    }
}
