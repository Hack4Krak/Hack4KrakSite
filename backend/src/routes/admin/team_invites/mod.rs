use crate::entities::external_team_invitation;
use crate::entities::team_invites;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod delete;
mod delete_external;
mod list;
mod list_by_team_id;

#[derive(Serialize, Deserialize, ToSchema)]
struct TeamInviteListResponse {
    team_invites: Vec<team_invites::Model>,
    external_team_invites: Vec<external_team_invitation::Model>,
}

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(delete::delete);
    config.service(list_by_team_id::list_by_team_id);
    config.service(delete_external::delete_external);
}
