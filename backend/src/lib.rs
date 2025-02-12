use crate::middlewares::status_code_drain_middleware::StatusCodeDrain;
use crate::utils::openapi::ApiDoc;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{App, Error};
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt, UtoipaApp};
use utoipa_scalar::{Scalar, Servable};

pub mod entities;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

pub fn setup_actix_app() -> UtoipaApp<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let governor_middleware = GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(5)
        .finish()
        .unwrap();

    let cors_middleware = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://hack4krak.pl")
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    App::new()
        .wrap(StatusCodeDrain)
        .wrap(Logger::default())
        .wrap(cors_middleware)
        .into_utoipa_app()
        .openapi(ApiDoc::openapi())
        .service(routes::index::index)
        .service(
            scope("/auth")
                .wrap(Governor::new(&governor_middleware))
                .configure(routes::auth::config),
        )
        .service(scope("/teams").configure(routes::teams::config))
        .service(scope("/tasks").configure(routes::task::config))
        .service(scope("/user").configure(routes::user::config))
        .openapi_service(|api| Scalar::with_url("/docs", api))
}
