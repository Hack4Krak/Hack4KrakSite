use crate::middlewares::auth::AuthMiddleware;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, error};
use hack4krak_macros::error_with_messages;
use utoipa_actix_web::scope;

mod confirm;
mod create;
mod external_invitations;
mod invitations;
mod management;
mod membership;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(create::create);
    config.service(confirm::confirm);
    config.service(scope("/external_invitations").configure(external_invitations::config));
    config.service(scope("/invitations").configure(invitations::config));
    config.service(
        scope("/membership")
            .wrap(AuthMiddleware::with_team_as_member())
            .configure(membership::config),
    );
    config.service(
        scope("/management")
            .wrap(AuthMiddleware::with_team_as_leader())
            .configure(management::config),
    );
}

#[error_with_messages]
pub enum TeamError {
    AlreadyExists,
    UserAlreadyBelongsToTeam { team_name: String },
    TeamNotFound,
    InvalidRegistrationPeriod,
    CannotRegisterInInternalMode,
    UserDoesntBelongToAnyTeam { username: String },
    UserDoesntHaveAnyInvitations,
    UserDoesntBelongToYourTeam,
    UserIsNotTeamLeader,
    UserCantRemoveYourself,
    UserCantRemoveTeamLeader,
    UserDoesntHaveInvitationsFromTeam { team_name: String },
    UserAlreadyInvited,
    TeamIsFull { max_size: u16 },
    TeamLeaderNotFound,
    InvalidNumberOfTeams { max_teams: usize },
}

impl error::ResponseError for TeamError {
    fn status_code(&self) -> StatusCode {
        match self {
            TeamError::InvalidRegistrationPeriod
            | TeamError::CannotRegisterInInternalMode
            | TeamError::InvalidNumberOfTeams { .. } => StatusCode::BAD_REQUEST,
            TeamError::AlreadyExists
            | TeamError::UserCantRemoveYourself
            | TeamError::TeamIsFull { .. }
            | TeamError::UserAlreadyInvited => StatusCode::CONFLICT,
            TeamError::UserAlreadyBelongsToTeam { .. }
            | TeamError::UserDoesntBelongToAnyTeam { .. }
            | TeamError::UserDoesntBelongToYourTeam
            | TeamError::UserIsNotTeamLeader
            | TeamError::UserCantRemoveTeamLeader => StatusCode::FORBIDDEN,
            TeamError::TeamNotFound
            | TeamError::UserDoesntHaveAnyInvitations
            | TeamError::UserDoesntHaveInvitationsFromTeam { .. }
            | TeamError::TeamLeaderNotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
