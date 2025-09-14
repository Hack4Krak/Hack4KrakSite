mod send_external_registration_form;
mod send_informational;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(send_informational::send_informational);
    config.service(send_external_registration_form::send_external_registration_form);
}
