use utoipa_actix_web::scope;

pub mod email;
mod tasks;
pub mod teams;
pub mod users;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(scope("/users").configure(users::config));
    config.service(scope("/teams").configure(teams::config));
    config.service(scope("/tasks").configure(tasks::config));
    config.service(scope("/email").configure(email::config));
}
