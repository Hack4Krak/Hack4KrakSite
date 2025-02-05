use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;

pub mod invitations;
mod leave_team;
pub mod management;
pub mod my_team;
pub mod team;
pub mod view_teams;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(view_teams::view_teams);
    cfg.service(team::view_team);
    cfg.service(my_team::my_team);
}

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("Team already exists")]
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
    #[error("Team is full")]
    TeamIsFull,
}

impl error::ResponseError for TeamError {
    fn status_code(&self) -> StatusCode {
        match self {
            TeamError::AlreadyExists
            | TeamError::UserCantRemoveYourself
            | TeamError::TeamIsFull => StatusCode::CONFLICT,
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
        crate::utils::error::json_error_response(self)
    }
}
