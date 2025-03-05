pub mod placeholders;
pub mod send;
pub mod templates;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(send::send);
    cfg.service(placeholders::placeholders);
    cfg.service(templates::templates);
}
