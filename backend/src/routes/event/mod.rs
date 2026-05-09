use utoipa_actix_web::scope;
use utoipa_actix_web::service_config::ServiceConfig;

mod info;
mod label;
mod participant_tags;
mod participate;
mod registration;
mod status;

pub use participate::ParticipationError;

pub fn config(config: &mut ServiceConfig) {
    config.service(info::info);
    config.service(status::status);
    config.service(registration::registration);
    config.service(label::label);
    config.service(participant_tags::participant_tags);
    config.service(scope("/participate").configure(participate::config));
}
