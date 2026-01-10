use utoipa_actix_web::service_config::ServiceConfig;

pub mod username_by_email;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(username_by_email::username_by_email);
}
