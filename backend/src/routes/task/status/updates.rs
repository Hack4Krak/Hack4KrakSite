use crate::entities::announcement;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, web};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct UpdatesQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[utoipa::path(
    params(UpdatesQuery),
    responses(
        (status = 200, description = "List of task status updates.", body = Vec<announcement::Model>),
    ),
    tag = "announcements"
)]
#[get("/task-status/updates")]
pub async fn updates(
    app_state: Data<AppState>,
    query: web::Query<UpdatesQuery>,
) -> Result<HttpResponse, Error> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);
    let models = announcement::Model::find_paginated(
        &app_state.database,
        Some("task_status_update"),
        limit,
        offset,
    )
    .await?;
    Ok(HttpResponse::Ok().json(models))
}
