mod delete_team;
mod teams_list;
pub mod update_team;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(teams_list::teams_list);
    cfg.service(update_team::update_team);
    cfg.service(delete_team::delete_team);
}
