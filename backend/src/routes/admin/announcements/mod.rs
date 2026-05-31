pub mod create;
pub mod list;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(create::create);
    config.service(list::list);
}
