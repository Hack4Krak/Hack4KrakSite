use serde::Deserialize;

pub mod github;
pub mod google;

#[derive(Deserialize, Debug)]
pub struct OAuthUser {
    name: String,
    email: String,
}
