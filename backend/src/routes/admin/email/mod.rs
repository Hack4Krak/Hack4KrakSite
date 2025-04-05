mod placeholders;
mod send;
mod templates;

pub use send::EmailSendingModel;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(send::send);
    config.service(placeholders::placeholders);
    config.service(templates::templates);
}
