use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;

use management::{add_user, change_leader, change_name, create_team, remove_user};

pub mod management;
pub mod my_team;
pub mod view_team;
pub mod view_teams;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(view_teams::view_teams);
    cfg.service(view_team::view_team);
    cfg.service(create_team::create_team);
    cfg.service(my_team::my_team);
    cfg.service(add_user::invite_user);
    cfg.service(add_user::get_invitations);
    cfg.service(add_user::accept_invitation);
    cfg.service(change_leader::change_leader);
    cfg.service(change_name::change_name);
    cfg.service(remove_user::remove_user);
}

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("Team already exists")]
    AlreadyExists,
    #[error("User already belongs to team: {team_name}")]
    UserAlreadyBelongsToTeam { team_name: String },
    #[error("Team not found")]
    TeamNotFound,
    #[error("User doesn't belong to any team")]
    UserDoesntBelongToAnyTeam,
    #[error("User doesn't have any invitations")]
    UserDoesntHaveAnyInvitations,
    #[error("User doesn't belong to your team")]
    UserDoesntBelongToYourTeam,
}

impl error::ResponseError for TeamError {
    fn status_code(&self) -> StatusCode {
        match self {
            TeamError::AlreadyExists => StatusCode::CONFLICT,
            TeamError::UserAlreadyBelongsToTeam { .. }
            | TeamError::UserDoesntBelongToAnyTeam
            | TeamError::UserDoesntBelongToYourTeam => StatusCode::FORBIDDEN,
            TeamError::TeamNotFound | TeamError::UserDoesntHaveAnyInvitations => {
                StatusCode::NOT_FOUND
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::json_error_response(self)
    }
}
