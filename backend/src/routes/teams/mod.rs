pub mod create_team;
pub mod my_team;
pub mod view_team;
pub mod view_teams;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(view_teams::view_teams);
    cfg.service(view_team::view_team);
    cfg.service(create_team::create_team);
    cfg.service(my_team::my_team);
}
