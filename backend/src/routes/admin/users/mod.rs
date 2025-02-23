mod delete;
mod email_confirmations_list;
mod list;
pub mod update;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(update::update);
    cfg.service(delete::delete);
    cfg.service(email_confirmations_list::email_confirmations_list);
}
