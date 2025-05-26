mod list;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(list::list_for_team);
    config.service(list::list_for_task);
}
