use utoipa_actix_web::scope;

mod tasks;
pub mod teams;
pub mod users;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(scope("/users").configure(users::config));
    cfg.service(scope("/teams").configure(teams::config));
}
