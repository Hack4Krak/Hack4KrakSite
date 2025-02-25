pub mod get_placeholders;
pub mod get_templates;
pub mod send;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(send::send_email);
    cfg.service(get_placeholders::get_placeholders);
    cfg.service(get_templates::get_templates);
}
