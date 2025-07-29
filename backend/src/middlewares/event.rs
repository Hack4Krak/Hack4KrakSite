use crate::entities::sea_orm_active_enums::UserRoles;
use crate::entities::users;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::{
    Error as ActixError, HttpMessage, ResponseError,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use chrono::Utc;
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::rc::Rc;

/// Restricts access to endpoints if event is not stared
#[derive(Clone, Copy)]
pub struct EventMiddleware {
    allow_before_event: bool,
    allow_during_event: bool,
    allow_after_event: bool,
}

impl EventMiddleware {
    pub fn new(before: bool, during: bool, after: bool) -> Self {
        Self {
            allow_before_event: before,
            allow_during_event: during,
            allow_after_event: after,
        }
    }

    pub fn allow_only_during_event() -> Self {
        Self::new(false, true, false)
    }

    pub fn allow_only_after_event() -> Self {
        Self::new(false, false, true)
    }

    pub fn disallow_before_event() -> Self {
        Self::new(false, true, true)
    }
}

impl<S, B> Transform<S, ServiceRequest> for EventMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = ActixError;
    type Transform = EventMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(EventMiddlewareService {
            service: Rc::new(service),
            middleware_config: *self,
        }))
    }
}

pub struct EventMiddlewareService<S> {
    service: Rc<S>,
    pub middleware_config: EventMiddleware,
}

impl<S, B> Service<ServiceRequest> for EventMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut request: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let middleware_config = self.middleware_config;

        Box::pin(async move {
            let response = Self::verify_request(&mut request, &middleware_config).await;
            if let Err(error) = response {
                return Ok(request
                    .into_response(error.error_response())
                    .map_into_right_body());
            }

            let response = service.call(request).await?;
            Ok(response.map_into_left_body())
        })
    }
}

impl<S> EventMiddlewareService<S> {
    async fn verify_request(
        request: &mut ServiceRequest,
        config: &EventMiddleware,
    ) -> Result<(), Error> {
        if let Some(user) = request.extensions().get::<users::Model>() {
            if user.roles >= UserRoles::Admin {
                return Ok(());
            }
        }

        let task_manager = &request
            .app_data::<web::Data<AppState>>()
            .ok_or(Error::Unauthorized)?
            .task_manager;

        let event_config = task_manager.event_config.read().await;

        let now = Utc::now();
        if !config.allow_before_event && now < event_config.start_date {
            return Err(Error::AccessBeforeEventStart {
                event_start_date: event_config.start_date.to_rfc3339(),
            });
        }
        if !config.allow_after_event && now > event_config.end_date {
            return Err(Error::AccessAfterEventEnd);
        }
        if !config.allow_during_event
            && event_config.start_date < now
            && now < event_config.end_date
        {
            return Err(Error::AccessDuringEvent);
        }

        Ok(())
    }
}
