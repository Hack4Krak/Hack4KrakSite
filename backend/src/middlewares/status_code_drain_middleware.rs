use crate::utils::error::ErrorHttpResponseExtension;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error as ActixError,
};
use futures::future::{ok, Ready};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::error;
use tracing::log::warn;

pub struct StatusCodeDrain;

impl<S, B> Transform<S, ServiceRequest> for StatusCodeDrain
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = StatusCodeDrainMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(StatusCodeDrainMiddleware { service })
    }
}

pub struct StatusCodeDrainMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for StatusCodeDrainMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let future = self.service.call(request);

        Box::pin(async move {
            let response = future.await?;
            let status = response.status();
            let path = response.request().path();
            let error = response
                .response()
                .extensions()
                .get::<ErrorHttpResponseExtension>()
                .map(|e| e.error.to_string())
                .unwrap_or_default();

            match status.as_u16() {
                400..=499 => {
                    warn!("Detected status: {} - {} - {{{:?}}}", status, path, error);
                }

                500..=599 => {
                    error!("Detected status: {} - {} - {{{:?}}}", status, path, error);
                }
                _ => {}
            }

            Ok(response)
        })
    }
}
