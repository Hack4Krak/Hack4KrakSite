use utoipa_actix_web::service_config::ServiceConfig;

pub mod change_password;
mod delete;
pub mod index;
pub mod update;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index);
    cfg.service(index::only_admins);
    cfg.service(delete::delete);
    cfg.service(change_password::change_password);
}
