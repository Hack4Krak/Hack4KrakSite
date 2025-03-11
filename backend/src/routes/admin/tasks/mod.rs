mod refresh;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(refresh::refresh);
}
