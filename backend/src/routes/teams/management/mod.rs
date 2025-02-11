pub mod change_leader;
pub mod invite_user;
pub mod kick_user;
pub mod rename;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(change_leader::change_leader)
        .service(invite_user::invite_user)
        .service(kick_user::kick_user)
        .service(rename::rename);
}
