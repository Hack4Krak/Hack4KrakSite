mod delete_user;
mod email_confirmations_list;
pub mod update_user;
mod user_list;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(user_list::user_list);
    cfg.service(update_user::update_user);
    cfg.service(delete_user::delete_user);
    cfg.service(email_confirmations_list::email_confirmations_list);
}
