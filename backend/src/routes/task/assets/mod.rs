mod get;
mod list;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(get::get);
}
