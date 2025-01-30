pub mod accept_invitation;
pub mod my_invites;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(accept_invitation::accept_invitation);
    cfg.service(my_invites::get_invitations);
}
