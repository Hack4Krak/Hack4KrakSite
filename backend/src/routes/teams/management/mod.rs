pub mod change_leader;
pub mod create_team;
pub mod invite_user;
pub mod remove_user;
pub mod rename;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(change_leader::change_leader)
        .service(create_team::create_team)
        .service(invite_user::invite_user)
        .service(remove_user::remove_user)
        .service(rename::rename);
}
