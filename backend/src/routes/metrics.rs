use crate::entities::{external_team_invitation, flag_capture, teams, user_personal_info, users};
use crate::services::env::EnvConfig;
use crate::utils::app_state::AppState;
use crate::utils::bearer::verify_bearer_token;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};
use prometheus::core::Collector;
use prometheus::proto::MetricFamily;
use prometheus::{Encoder, Gauge, Opts, TextEncoder};
use sea_orm::PaginatorTrait;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/metrics")]
pub async fn metrics(
    app_state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    verify_bearer_token(&request, &EnvConfig::get().metrics_access_token)?;

    let users_in_teams = &users::Entity::find()
        .filter(users::Column::Team.is_not_null())
        .count(&app_state.database)
        .await?;

    let metric_families = [
        add_simple_metric::<users::Entity>("app_count_users", &app_state).await?,
        add_metric("app_count_users_in_teams", *users_in_teams).await?,
        add_simple_metric::<teams::Entity>("app_count_teams", &app_state).await?,
        add_simple_metric::<user_personal_info::Entity>("app_count_personal_info", &app_state)
            .await?,
        add_simple_metric::<flag_capture::Entity>("app_count_flag_capture", &app_state).await?,
        add_simple_metric::<external_team_invitation::Entity>(
            "app_count_external_team_invitation",
            &app_state,
        )
        .await?,
    ]
    .concat();

    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Ok(HttpResponse::Ok().body(buffer))
}

async fn add_simple_metric<T>(
    metric_name: &str,
    app_state: &AppState,
) -> Result<Vec<MetricFamily>, Error>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let gauge = Gauge::with_opts(Opts::new(
        metric_name,
        format!("Current number of {metric_name}"),
    ))?;

    let count =
        <sea_orm::Select<T> as PaginatorTrait<_>>::count(T::find(), &app_state.database).await?;

    gauge.set(count as f64);
    Ok(gauge.collect())
}

async fn add_metric(metric_name: &str, count: u64) -> Result<Vec<MetricFamily>, Error> {
    let gauge = Gauge::with_opts(Opts::new(
        metric_name,
        format!("Current number of {metric_name}"),
    ))?;

    gauge.set(count as f64);
    Ok(gauge.collect())
}
