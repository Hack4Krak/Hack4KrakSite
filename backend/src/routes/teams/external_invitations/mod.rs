mod create;
mod info;
mod join;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(create::create);
    config.service(join::join);
    config.service(info::info);
}
