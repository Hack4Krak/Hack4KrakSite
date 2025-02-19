use crate::entities::sea_orm_active_enums::UserRoles;
use crate::entities::users::ActiveModel;
use crate::entities::{teams, users};
use crate::models::task::EventConfig;
use crate::routes::admin::users::update::UpdateUserModel;
use crate::routes::auth::AuthError::UserAlreadyExists;
use crate::routes::auth::RegisterModel;
use crate::routes::teams::TeamError::TeamNotFound;
use crate::utils::error::Error;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use chrono::Local;
use sea_orm::prelude::Uuid as SeaOrmUuid;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::{ColumnTrait, DatabaseConnection};
use sea_orm::{ModelTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::future;
use std::str::FromStr;
use uuid::{Uuid as uuid_gen, Uuid};

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
        users::Model::assert_is_unique(database, &credentials.email, &credentials.name, None)
            .await?;

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

    pub async fn find_by_uuid_from_string(
        database: &DatabaseConnection,
        id: &str,
    ) -> Result<Self, Error> {
        let user = users::Entity::find_by_id(Uuid::parse_str(id).map_err(|_| Error::UserNotFound)?)
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

        Ok(user)
    }

    pub async fn assert_is_unique(
        database: &DatabaseConnection,
        email: &str,
        username: &str,
        id: Option<SeaOrmUuid>,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(
                users::Column::Email
                    .eq(email)
                    .or(users::Column::Username.eq(username)),
            )
            .one(database)
            .await?;

        let same_user_exists = user.is_some();

        if let (Some(user), Some(id)) = (user, id) {
            if user.id != id {
                return Err(Error::UserWithEmailOrUsernameAlreadyExists);
            }
            return Ok(());
        }

        if same_user_exists {
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
                users::Model::assert_is_unique(database, &email, &username, None).await?;

                ActiveModel {
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
        users::Model::assert_is_unique(database, &user_info.email, &user_info.name, None).await?;

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

    pub async fn update(
        database: &DatabaseConnection,
        user: users::Model,
        event_config: &EventConfig,
        id: SeaOrmUuid,
        update_user_json: UpdateUserModel,
    ) -> Result<(), Error> {
        let updated_user = users::Entity::find_by_id(id)
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

        if user.roles.permission_level() <= updated_user.roles.permission_level() {
            return Err(Error::UserMustHaveHigherRoleThanAffectedUser);
        }

        let mut active_user: ActiveModel = updated_user.into();

        if let Some(username) = update_user_json.username {
            active_user.username = Set(username);
        }

        if let Some(email) = update_user_json.email {
            active_user.email = Set(email);
        }

        if let Some(team) = update_user_json.team {
            let team = uuid_gen::from_str(&team).map_err(|_| TeamNotFound)?;
            teams::Model::assert_correct_team_size(
                database,
                event_config.max_team_size,
                &team.clone(),
            )
            .await?;
            active_user.team = Set(Some(team));
        }

        if Self::assert_is_unique(
            database,
            &active_user.email.clone().unwrap(),
            &active_user.username.clone().unwrap(),
            Some(id),
        )
        .await
        .is_err()
        {
            return Err(Error::UserWithEmailOrUsernameAlreadyExists);
        }

        active_user.save(database).await?;
        Ok(())
    }

    pub async fn delete(
        database: &DatabaseConnection,
        user: users::Model,
        id: SeaOrmUuid,
    ) -> Result<(), Error> {
        let user_to_delete = users::Entity::find_by_id(id)
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

        if user.roles.permission_level() <= user_to_delete.roles.permission_level() {
            return Err(Error::UserMustHaveHigherRoleThanAffectedUser);
        }

        user_to_delete.delete(database).await?;

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
