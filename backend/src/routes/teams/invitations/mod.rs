pub mod accept_invitation;
pub mod my_invites;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(accept_invitation::accept_invitation);
    config.service(my_invites::get_invitations);
}
