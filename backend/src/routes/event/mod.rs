mod info;
mod label;
mod registration;
mod status;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(info::info);
    config.service(status::status);
    config.service(registration::registration);
    config.service(label::label);
}
