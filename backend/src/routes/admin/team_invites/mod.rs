mod delete;
mod list;
mod list_for_team;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list_for_team::list_for_team);
    config.service(list::list);
    config.service(list::list_external);
    config.service(delete::delete_external);
    config.service(delete::delete);
}
