use crate::middlewares::auth::AuthMiddleware;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;
use utoipa_actix_web::scope;

mod create;
mod invitations;
mod management;
mod membership;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(create::create);
    cfg.service(
        scope("/invitations")
            .wrap(AuthMiddleware::with_user())
            .configure(invitations::config),
    );
    cfg.service(
        scope("/membership")
            .wrap(AuthMiddleware::with_team_as_member())
            .configure(membership::config),
    );
    cfg.service(
        scope("/management")
            .wrap(AuthMiddleware::with_team_as_leader())
            .configure(management::config),
    );
}

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("Team with same name already exists")]
    AlreadyExists,
    #[error("User already belongs to team: {team_name}")]
    UserAlreadyBelongsToTeam { team_name: String },
    #[error("Team not found")]
    TeamNotFound,
    #[error("User {username} doesn't belong to any team")]
    UserDoesntBelongToAnyTeam { username: String },
    #[error("User doesn't have any invitations")]
    UserDoesntHaveAnyInvitations,
    #[error("User doesn't belong to your team")]
    UserDoesntBelongToYourTeam,
    #[error("User is not team leader")]
    UserIsNotTeamLeader,
    #[error("You can't remove yourself from the team")]
    UserCantRemoveYourself,
    #[error("You can't remove the team leader")]
    UserCantRemoveTeamLeader,
    #[error("User doesn't have invitations from {team_name}")]
    UserDoesntHaveInvitationsFromTeam { team_name: String },
    #[error("Team leader can't leave team")]
    TeamLeaderCantLeaveTeam,
    #[error("User already has an invitation to this team")]
    UserAlreadyInvited,
    #[error("Team is full. Max team size is {max_size}")]
    TeamIsFull { max_size: u16 },
}

impl error::ResponseError for TeamError {
    fn status_code(&self) -> StatusCode {
        match self {
            TeamError::AlreadyExists
            | TeamError::UserCantRemoveYourself
            | TeamError::TeamIsFull { .. }
            | TeamError::UserAlreadyInvited => StatusCode::CONFLICT,
            TeamError::UserAlreadyBelongsToTeam { .. }
            | TeamError::UserDoesntBelongToAnyTeam { .. }
            | TeamError::UserDoesntBelongToYourTeam
            | TeamError::UserIsNotTeamLeader
            | TeamError::UserCantRemoveTeamLeader
            | TeamError::TeamLeaderCantLeaveTeam => StatusCode::FORBIDDEN,
            TeamError::TeamNotFound
            | TeamError::UserDoesntHaveAnyInvitations
            | TeamError::UserDoesntHaveInvitationsFromTeam { .. } => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
