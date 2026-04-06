use utoipa_actix_web::scope;

mod email;
mod index;
mod tasks;
mod teams;
mod users;
mod verification;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(scope("/users").configure(users::config));
    config.service(scope("/teams").configure(teams::config));
    config.service(scope("/tasks").configure(tasks::config));
    config.service(scope("/email").configure(email::config));
    config.service(scope("/verification").configure(verification::config));
    config.service(index::index);
}
