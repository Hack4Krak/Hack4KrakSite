use utoipa_actix_web::service_config::ServiceConfig;

mod capture_log;
mod team_standings;

pub fn config(config: &mut ServiceConfig) {
    config.service(team_standings::team_standings);
    config.service(capture_log::capture_log);
}
