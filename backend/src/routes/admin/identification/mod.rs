mod apply_tag;
mod identify;
mod reset_identification_code;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(identify::identify);
    config.service(apply_tag::apply_tag);
    config.service(reset_identification_code::reset_identification_code);
}
