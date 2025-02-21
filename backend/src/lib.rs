use crate::middlewares::event::EventMiddleware;
use crate::middlewares::status_code_drain_middleware::StatusCodeDrain;
use crate::services::env::EnvConfig;
use crate::utils::error::Error::RouteNotFound;
use crate::utils::openapi::ApiDoc;
use actix_cors::Cors;
use actix_governor::governor::clock::QuantaInstant;
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_governor::{Governor, GovernorConfig, GovernorConfigBuilder, PeerIpKeyExtractor};
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::{App, Error, ResponseError};
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt, UtoipaApp};
use utoipa_scalar::{Scalar, Servable};

pub mod entities;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

pub fn setup_actix_app(
    enable_governor: bool,
) -> UtoipaApp<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let cors_middleware = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://hack4krak.pl")
        .allowed_origin_fn(|origin, request| {
            let Ok(origin) = origin.to_str() else {
                return false;
            };

            if EnvConfig::get().relaxed_security_mode
                && origin.starts_with("https://hack4krak")
                && origin.ends_with("-spacemceu.vercel.app")
                && !request.uri.path().starts_with("/admin")
            {
                return true;
            }

            false
        })
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    let mut app = App::new()
        .wrap(StatusCodeDrain)
        .wrap(Logger::default())
        .wrap(cors_middleware)
        .into_utoipa_app()
        .openapi(ApiDoc::openapi())
        .service(routes::index::index)
        .service(scope("/auth").configure(routes::auth::config))
        .service(scope("/teams").configure(routes::teams::config))
        .service(
            scope("/tasks")
                .wrap(EventMiddleware::allow_after_event())
                .configure(routes::task::config),
        )
        .service(scope("/user").configure(routes::user::config))
        .service(scope("/event").configure(routes::event::config))
        .service(scope("/flag").configure(routes::flag::config))
        .default_service(actix_web::web::route().to(|| async { RouteNotFound.error_response() }))
        .openapi_service(|api| Scalar::with_url("/docs", api));

    if enable_governor {
        let auth_governor: GovernorConfig<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>> =
            GovernorConfig::secure();
        app = app.service(
            scope("/auth")
                .wrap(Governor::new(&auth_governor))
                .configure(routes::auth::config),
        );

        let flag_governor = GovernorConfigBuilder::default()
            .seconds_per_request(5)
            .burst_size(2)
            .finish()
            .unwrap();
        app = app.service(
            scope("/flag")
                .wrap(Governor::new(&flag_governor))
                .wrap(EventMiddleware::allow_during_event_only())
                .configure(routes::flag::config),
        );
    }

    app
}
