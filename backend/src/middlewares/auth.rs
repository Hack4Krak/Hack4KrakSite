use crate::entities::sea_orm_active_enums::UserRoles;
use crate::entities::{teams, users};
use crate::routes::teams::TeamError;
use crate::utils::app_state::AppState;
use crate::utils::cookies::ACCESS_TOKEN_COOKIE;
use crate::utils::error::Error;
use crate::utils::jwt::decode_jwt;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::{
    Error as ActixError, HttpMessage, ResponseError,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::LocalBoxFuture;
use sea_orm::EntityTrait;
use std::cmp::PartialEq;
use std::future::{Ready, ready};
use std::rc::Rc;

/// Middleware responsible for verifying authenticated users
/// It also has options to insert corresponding extensions to the request
#[derive(Default, Clone, Copy)]
pub struct AuthMiddleware {
    insert_user_extension: bool,
    insert_team_extension: bool,
    team_requirement: TeamRequirement,
    role_requirement: UserRoles,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum TeamRequirement {
    #[default]
    None,
    Member,
    Leader,
}

impl AuthMiddleware {
    pub fn with_user() -> Self {
        AuthMiddleware {
            insert_user_extension: true,
            ..Default::default()
        }
    }

    pub fn with_team_as_member() -> Self {
        AuthMiddleware {
            insert_user_extension: true,
            insert_team_extension: true,
            team_requirement: TeamRequirement::Member,
            role_requirement: UserRoles::Default,
        }
    }

    pub fn with_team_as_leader() -> Self {
        AuthMiddleware {
            insert_user_extension: true,
            insert_team_extension: true,
            team_requirement: TeamRequirement::Leader,
            role_requirement: UserRoles::Default,
        }
    }

    pub fn with_user_as_admin() -> Self {
        AuthMiddleware {
            insert_user_extension: true,
            role_requirement: UserRoles::Admin,
            ..Default::default()
        }
    }

    pub fn with_user_as_owner() -> Self {
        AuthMiddleware {
            insert_user_extension: true,
            role_requirement: UserRoles::Owner,
            ..Default::default()
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = ActixError;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            middleware_config: *self,
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    pub middleware_config: AuthMiddleware,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let config = self.middleware_config;

        Box::pin(async move {
            let response = Self::handle_middleware_request(&mut request, &config).await;
            if let Err(error) = response {
                return Ok(request
                    .into_response(error.error_response())
                    .map_into_right_body());
            }

            let response = service.call(request).await?;
            Ok(response.map_into_left_body())
        })
    }
}

impl<S> AuthMiddlewareService<S> {
    async fn handle_middleware_request(
        request: &mut ServiceRequest,
        config: &AuthMiddleware,
    ) -> Result<(), Error> {
        let cookie = request
            .cookie(ACCESS_TOKEN_COOKIE)
            .ok_or(Error::Unauthorized)?;

        let claims = decode_jwt(cookie.value()).map_err(|_| Error::Unauthorized)?;

        let user_id = claims.claims.id;
        request.extensions_mut().insert(claims.claims);

        if !config.insert_user_extension {
            return Ok(());
        }

        let database = &request
            .app_data::<web::Data<AppState>>()
            .ok_or(Error::Unauthorized)?
            .database;

        let user = users::Entity::find_by_id(user_id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        match config.team_requirement {
            TeamRequirement::None => (),
            TeamRequirement::Leader | TeamRequirement::Member => {
                let Some(team) = user.team else {
                    return Err(TeamError::UserDoesntBelongToAnyTeam {
                        username: user.username,
                    }
                    .into());
                };
                if !user.is_leader && config.team_requirement == TeamRequirement::Leader {
                    return Err(TeamError::UserIsNotTeamLeader.into());
                }
                if config.insert_team_extension {
                    let team = teams::Entity::find_by_id(team)
                        .one(database)
                        .await?
                        .ok_or(Error::Team(TeamError::TeamNotFound))?;

                    request.extensions_mut().insert(team);
                }
            }
        }

        match config.role_requirement {
            UserRoles::Default => (),
            UserRoles::Admin => {
                if user.roles < UserRoles::Admin {
                    return Err(Error::Forbidden {
                        required_role: UserRoles::Admin,
                    });
                }
            }
            UserRoles::Owner => {
                if user.roles < UserRoles::Owner {
                    return Err(Error::Forbidden {
                        required_role: UserRoles::Owner,
                    });
                }
            }
        }

        request.extensions_mut().insert(user);
        Ok(())
    }
}
