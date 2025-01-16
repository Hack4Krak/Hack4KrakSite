mod login;
mod refresh;
mod register;

pub use login::LoginModel;
pub use register::RegisterModel;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(register::register);
    cfg.service(login::login);
    cfg.service(refresh::refresh);
}
