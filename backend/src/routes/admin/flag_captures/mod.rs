use crate::entities::flag_capture;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod list;
mod list_by_task;
mod list_by_team_id;

// This struct is needed for OpenAPI to work properly
// Otherwise it recognize every SeaOrm Model as same type, and it messes up the OpenAPI schema

#[derive(Serialize, Deserialize, ToSchema)]
struct FlagCaptureListResponse {
    flag_captures: Vec<flag_capture::Model>,
}

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(list_by_team_id::list_by_team_id);
    config.service(list_by_task::list_by_task);
}
