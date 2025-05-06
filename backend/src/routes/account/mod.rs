use utoipa_actix_web::service_config::ServiceConfig;

mod delete;
mod get_personal_info;
pub mod index;
mod submit_personal_information;
pub mod update;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index);
    cfg.service(index::only_admins);
    cfg.service(delete::delete);
    cfg.service(update::update);
    cfg.service(update::change_password);
    cfg.service(submit_personal_information::submit_personal_information);
    cfg.service(get_personal_info::get_personal_information);
}
