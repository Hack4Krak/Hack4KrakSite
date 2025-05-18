use crate::entities::{external_team_invitation, teams, user_personal_info, users};
use crate::services::env::EnvConfig;
use crate::utils::app_state::AppState;
use crate::utils::bearer::verify_bearer_token;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};
use prometheus::core::Collector;
use prometheus::proto::MetricFamily;
use prometheus::{Encoder, Gauge, Opts, TextEncoder};
use sea_orm::EntityTrait;

#[utoipa::path(responses((status = OK, body = String)))]
#[get("/metrics")]
pub async fn metrics(
    app_state: web::Data<AppState>,
    request: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    verify_bearer_token(&request, &EnvConfig::get().metrics_access_token)?;

    let metric_families = [
        add_metric::<users::Entity>("app_count_users", &app_state).await?,
        add_metric::<teams::Entity>("app_count_teams", &app_state).await?,
        add_metric::<user_personal_info::Entity>("app_count_personal_info", &app_state).await?,
        add_metric::<external_team_invitation::Entity>(
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

async fn add_metric<T>(metric_name: &str, app_state: &AppState) -> Result<Vec<MetricFamily>, Error>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let gauge = Gauge::with_opts(Opts::new(
        metric_name,
        format!("Current number of {metric_name}"),
    ))?;

    let count =
        <sea_orm::Select<T> as sea_orm::PaginatorTrait<_>>::count(T::find(), &app_state.database)
            .await?;

    gauge.set(count as f64);
    Ok(gauge.collect())
}
