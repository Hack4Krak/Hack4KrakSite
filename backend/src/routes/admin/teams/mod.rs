mod clear_confirmation_code;
mod delete;
mod generate_confirmation_code;
mod list;
pub mod update;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(update::update);
    cfg.service(delete::delete);
}
