mod delete;
mod email_confirmations_list;
mod list;
mod update;

pub use update::UpdateUserModelAdmin;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(update::update);
    config.service(delete::delete);
    config.service(email_confirmations_list::email_confirmations_list);
}
