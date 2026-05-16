use crate::entities::sea_orm_active_enums::TeamStatus;
use crate::entities::{
    event_registration, external_team_invitation, flag_capture, teams, user_onboarding, users,
};
use crate::services::env::EnvConfig;
use crate::utils::app_state::AppState;
use crate::utils::bearer::verify_bearer_token;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};
use prometheus::core::Collector;
use prometheus::proto::MetricFamily;
use prometheus::{Encoder, Gauge, GaugeVec, Opts, TextEncoder};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/metrics")]
pub async fn metrics(
    app_state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    verify_bearer_token(&request, &EnvConfig::get().metrics_access_token)?;

    let database = &app_state.database;

    let users_total = count::<users::Entity>(database).await?;
    let users_in_teams = users::Entity::find()
        .filter(users::Column::Team.is_not_null())
        .count(database)
        .await?;
    let users_teamless = users::Entity::find()
        .filter(users::Column::Team.is_null())
        .count(database)
        .await?;

    let unique_tasks_solved = flag_capture::Entity::find()
        .select_only()
        .expr(Expr::cust("COUNT(DISTINCT task)"))
        .into_tuple::<i64>()
        .one(database)
        .await?
        .unwrap_or(0) as u64;

    let teams_confirmed = teams::Entity::find()
        .filter(teams::Column::Status.eq(TeamStatus::Confirmed))
        .count(database)
        .await?;
    let teams_absent = teams::Entity::find()
        .filter(teams::Column::Status.eq(TeamStatus::Absent))
        .count(database)
        .await?;

    let metric_families = [
        gauge("app_count_users", users_total)?,
        gauge("app_count_users_in_teams", users_in_teams)?,
        gauge("app_count_users_teamless", users_teamless)?,
        gauge("app_count_tasks", app_state.task_manager.tasks.len() as u64)?,
        gauge("app_count_teams", teams_confirmed + teams_absent)?,
        gauge_vec(
            "app_count_teams_by_status",
            "status",
            &[("confirmed", teams_confirmed), ("absent", teams_absent)],
        )?,
        gauge(
            "app_count_onboarding",
            count::<user_onboarding::Entity>(database).await?,
        )?,
        gauge(
            "app_count_flag_capture",
            count::<flag_capture::Entity>(database).await?,
        )?,
        gauge("app_count_flag_capture_unique_tasks", unique_tasks_solved)?,
        gauge(
            "app_count_external_team_invitation",
            count::<external_team_invitation::Entity>(database).await?,
        )?,
        gauge(
            "app_count_event_registrations",
            count::<event_registration::Entity>(database).await?,
        )?,
        gauge(
            "app_count_sse_receivers",
            app_state.sse_event_sender.receiver_count() as u64,
        )?,
    ]
    .concat();

    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(HttpResponse::Ok().body(buffer))
}

async fn count<T>(database: &DatabaseConnection) -> Result<u64, Error>
where
    T: EntityTrait,
    T::Model: Sync,
{
    Ok(<sea_orm::Select<T> as PaginatorTrait<_>>::count(T::find(), database).await?)
}

fn gauge(name: &str, value: u64) -> Result<Vec<MetricFamily>, Error> {
    let gauge_metric = Gauge::with_opts(Opts::new(name, format!("Current number of {name}")))?;
    gauge_metric.set(value as f64);
    Ok(gauge_metric.collect())
}

fn gauge_vec(name: &str, label: &str, entries: &[(&str, u64)]) -> Result<Vec<MetricFamily>, Error> {
    let gauge_metric = GaugeVec::new(
        Opts::new(name, format!("Current number of {name}")),
        &[label],
    )?;
    for (label_value, count) in entries {
        gauge_metric
            .with_label_values(&[label_value])
            .set(*count as f64);
    }
    Ok(gauge_metric.collect())
}
