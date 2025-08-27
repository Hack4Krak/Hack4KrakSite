use askama::Template;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/email_confirmation.html")]
pub struct EmailConfirmation {
    pub link: String,
    pub user: String,
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/reset_password.html")]
pub struct ResetPassword {
    pub link: String,
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/informational.html")]
pub struct Informational {
    pub title: String,
    pub content: String,
}

#[derive(Template, Serialize, Deserialize, ToSchema)]
#[template(path = "email/external_registration_form.html")]
pub struct ExternalRegistrationForm {
    pub organization: String,
    pub link: String,
}
