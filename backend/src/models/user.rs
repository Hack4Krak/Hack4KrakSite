use crate::entities::sea_orm_active_enums::UserRoles;
use crate::entities::{teams, users};
use crate::routes::auth::AuthError::UserAlreadyExists;
use crate::routes::auth::RegisterModel;
use crate::utils::error::Error;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait, DatabaseConnection};

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::future;
use uuid::Uuid as uuid_gen;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInformation {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl UserInformation {
    pub async fn new(
        database: &DatabaseConnection,
        password_hash: String,
        credentials: &RegisterModel,
    ) -> Result<UserInformation, Error> {
        users::Model::assert_is_unique(database, &credentials.email, &credentials.name).await?;

        let user_info = UserInformation {
            name: credentials.name.clone(),
            email: credentials.email.clone(),
            password_hash: password_hash.clone(),
        };

        Ok(user_info)
    }
}

impl users::Model {
    pub async fn find_by_username(
        database: &DatabaseConnection,
        name: &str,
    ) -> Result<Option<Self>, Error> {
        Ok(users::Entity::find()
            .filter(users::Column::Username.eq(name))
            .one(database)
            .await?)
    }

    pub async fn find_by_email(
        database: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<Self>, Error> {
        Ok(users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(database)
            .await?)
    }

    pub async fn assert_is_unique(
        database: &DatabaseConnection,
        email: &str,
        username: &str,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(
                users::Column::Email
                    .eq(email)
                    .or(users::Column::Username.eq(username)),
            )
            .one(database)
            .await?;

        if user.is_some() {
            return Err(Error::Auth(UserAlreadyExists));
        }

        Ok(())
    }

    pub async fn get_team(
        &self,
        database: &DatabaseConnection,
    ) -> Result<Option<teams::Model>, Error> {
        if let Some(team_id) = self.team {
            return Ok(teams::Entity::find_by_id(team_id).one(database).await?);
        }

        Ok(None)
    }

    pub async fn create_from_oauth(
        database: &DatabaseConnection,
        username: String,
        email: String,
    ) -> Result<Self, Error> {
        let user = users::Model::find_by_email(database, &email).await?;

        let user = match user {
            Some(user) => user,
            None => {
                users::Model::assert_is_unique(database, &email, &username).await?;

                users::ActiveModel {
                    id: Set(uuid_gen::new_v4()),
                    username: Set(username),
                    email: Set(email.clone()),
                    password: Set(None),
                    ..Default::default()
                }
                .insert(database)
                .await?
            }
        };

        Ok(user)
    }

    pub async fn create_from_user_info(
        database: &DatabaseConnection,
        user_info: UserInformation,
    ) -> Result<(), Error> {
        users::Model::assert_is_unique(database, &user_info.email, &user_info.name).await?;

        users::ActiveModel {
            id: Set(uuid_gen::new_v4()),
            username: Set(user_info.name.clone()),
            email: Set(user_info.email.clone()),
            password: Set(Some(user_info.password_hash.clone())),
            created_at: Set(Local::now().naive_local()),
            is_leader: Set(false),
            roles: Set(UserRoles::Default),
            ..Default::default()
        }
        .insert(database)
        .await?;

        Ok(())
    }
}

impl FromRequest for users::Model {
    type Error = Error;

    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.extensions().get::<users::Model>() {
            Some(data) => future::ready(Ok(data.clone())),
            None => future::ready(Err(Error::MissingExtension {
                name: "user".to_string(),
            })),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for UserRoles {
    fn default() -> Self {
        UserRoles::Default
    }
}

impl UserRoles {
    pub fn permission_level(&self) -> u8 {
        match self {
            UserRoles::Owner => 2,
            UserRoles::Admin => 1,
            UserRoles::Default => 0,
        }
    }
}
