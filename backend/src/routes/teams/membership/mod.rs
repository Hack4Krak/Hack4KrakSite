use utoipa_actix_web::service_config::ServiceConfig;

mod completed_tasks;
mod leave_team;
mod my_team;
mod stats;

pub fn config(config: &mut ServiceConfig) {
    config
        .service(leave_team::leave_team)
        .service(my_team::my_team)
        .service(completed_tasks::completed_tasks)
        .service(stats::stats);
}
