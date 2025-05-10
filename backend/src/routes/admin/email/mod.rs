mod placeholders;
mod send;
mod send_external_registration_form;
mod templates;

pub use send::EmailSendingModel;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(send::send);
    config.service(placeholders::placeholders);
    config.service(send_external_registration_form::send_external_registration_form);
    config.service(templates::templates);
}
