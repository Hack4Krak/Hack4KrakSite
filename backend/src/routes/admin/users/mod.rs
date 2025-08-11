mod delete;
mod email_verification_request_list;
mod get_personal_information;
mod list;
mod update;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(update::update);
    config.service(delete::delete);
    config.service(email_verification_request_list::email_confirmations_list);
    config.service(get_personal_information::get_personal_information);
}
