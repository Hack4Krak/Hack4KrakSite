mod create;
mod info;
mod join;
mod list;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(create::create);
    config.service(join::join);
    config.service(list::list);
    config.service(info::info);
}
