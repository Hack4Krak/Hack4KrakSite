mod get;
mod list;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(get::get);
}
