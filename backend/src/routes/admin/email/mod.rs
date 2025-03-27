mod placeholders;
pub mod send;
mod templates;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(send::send);
    config.service(placeholders::placeholders);
    config.service(templates::templates);
}
