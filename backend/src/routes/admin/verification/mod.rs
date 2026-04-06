mod apply_tag;
mod identify;
mod reset_uuid;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(identify::identify);
    config.service(apply_tag::apply_tag);
    config.service(reset_uuid::reset_uuid);
}
