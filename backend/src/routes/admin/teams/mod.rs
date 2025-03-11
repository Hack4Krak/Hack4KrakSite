mod clear_confirmation_code;
mod delete;
mod generate_confirmation_code;
mod list;
pub mod update;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(update::update);
    config.service(delete::delete);
}
