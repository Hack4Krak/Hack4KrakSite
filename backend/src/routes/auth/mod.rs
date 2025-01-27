mod login;
mod oauth;
mod refresh;
mod register;

pub use login::LoginModel;
pub use login::TokensResponse;
pub use register::RegisterModel;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(register::register);
    cfg.service(login::login);
    cfg.service(refresh::refresh);
    cfg.service(oauth::github::github);
    cfg.service(oauth::github::github_callback);
    cfg.service(oauth::google::google_callback);
    cfg.service(oauth::google::google);
}
