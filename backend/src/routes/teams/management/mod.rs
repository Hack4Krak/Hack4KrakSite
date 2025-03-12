pub mod change_leader;
mod delete;
mod index;
pub mod invite_user;
mod invited_users;
pub mod kick_user;
pub mod rename;
mod revoke_invitation;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config
        .service(change_leader::change_leader)
        .service(invite_user::invite_user)
        .service(kick_user::kick_user)
        .service(rename::rename)
        .service(index::index)
        .service(delete::delete_team)
        .service(invited_users::invited_users)
        .service(revoke_invitation::revoke_invitation);
}
