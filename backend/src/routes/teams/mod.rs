use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;

pub mod create_team;
pub mod my_team;
pub mod view_team;
pub mod view_teams;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(view_teams::view_teams);
    cfg.service(view_team::view_team);
    cfg.service(create_team::create_team);
    cfg.service(my_team::my_team);
}

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("Team already exists")]
    AlreadyExists,
    #[error("User already belongs to team: {team_name}")]
    UserAlreadyBelongsToTeam { team_name: String },
    #[error("Team not found")]
    NotFound,
    #[error("User doesn't belong to any team")]
    UserDoesntBelongToAnyTeam,
}

impl error::ResponseError for TeamError {
    fn status_code(&self) -> StatusCode {
        match self {
            TeamError::AlreadyExists => StatusCode::CONFLICT,
            TeamError::UserAlreadyBelongsToTeam { .. } | TeamError::UserDoesntBelongToAnyTeam => {
                StatusCode::FORBIDDEN
            }
            TeamError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::json_error_response(self)
    }
}
