mod info;
mod is_event_in_progress;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(info::info);
    cfg.service(is_event_in_progress::is_event_in_progress);
}
