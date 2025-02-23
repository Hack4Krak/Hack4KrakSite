mod info;
mod status;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(info::info);
    cfg.service(status::status);
}
