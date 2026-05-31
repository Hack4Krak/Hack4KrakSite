use crate::entities::announcement;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, web};
use serde::Deserialize;
use utoipa::IntoParams;

const MAX_LIMIT: u64 = 100;

#[derive(Deserialize, IntoParams)]
pub struct ListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub action_type: Option<String>,
}

#[utoipa::path(
    params(ListQuery),
    responses(
        (status = 200, description = "List of announcements.", body = Vec<announcement::Model>),
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin-announcements-list",
    tag = "admin/announcements"
)]
#[get("/")]
pub async fn list(
    app_state: Data<AppState>,
    query: web::Query<ListQuery>,
) -> Result<HttpResponse, Error> {
    let limit = query.limit.unwrap_or(20).min(MAX_LIMIT);
    let offset = query.offset.unwrap_or(0);
    let action_type = query.action_type.as_deref();
    let models =
        announcement::Model::find_paginated(&app_state.database, action_type, limit, offset)
            .await?;
    Ok(HttpResponse::Ok().json(models))
}
