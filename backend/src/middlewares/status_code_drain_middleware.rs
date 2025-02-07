use actix_web::http::StatusCode;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use log::error;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StatusCodeDrain;

impl<S, B> Transform<S, ServiceRequest> for StatusCodeDrain
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let status = res.status();
            let path = res.request().path();

            if status == StatusCode::INTERNAL_SERVER_ERROR
                || status == StatusCode::BAD_REQUEST
                || status == StatusCode::BAD_GATEWAY
                || status == StatusCode::SERVICE_UNAVAILABLE
                || status == StatusCode::GATEWAY_TIMEOUT
                || status == StatusCode::REQUEST_TIMEOUT
            {
                error!("Detected status: {} - {}", status, path);
            }

            Ok(res)
        })
    }
}
