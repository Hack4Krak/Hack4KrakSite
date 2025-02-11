use utoipa_actix_web::service_config::ServiceConfig;

pub mod leave_team;
pub mod my_team;

pub fn config(config: &mut ServiceConfig) {
    config
        .service(leave_team::leave_team)
        .service(my_team::my_team);
}
