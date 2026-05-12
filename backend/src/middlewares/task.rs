use crate::routes::task::TaskError;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::{
    Error as ActixError, ResponseError,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use chrono::Utc;
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct TaskReleasePhaseMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TaskReleasePhaseMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = ActixError;
    type Transform = TaskReleasePhaseMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TaskReleasePhaseMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct TaskReleasePhaseMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TaskReleasePhaseMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            if let Some(task_id) = request.match_info().get("task_id").map(String::from)
                && let Err(e) = check_task_release_phase(&request, &task_id).await
            {
                return Ok(request
                    .into_response(e.error_response())
                    .map_into_right_body());
            }

            let response = service.call(request).await?;
            Ok(response.map_into_left_body())
        })
    }
}

async fn check_task_release_phase(request: &ServiceRequest, task_id: &str) -> Result<(), Error> {
    let app_state = request
        .app_data::<web::Data<AppState>>()
        .ok_or(Error::Unauthorized)?;

    let event_config = app_state.task_manager.event_config.read().await;
    let task = app_state.task_manager.get_task(task_id)?;
    let phase_time = event_config
        .task_release_phases
        .get(&task.task_release_phase);
    if phase_time.is_none_or(|time| time > &Utc::now()) {
        return Err(TaskError::MissingTask {
            id: task_id.to_string(),
        }
        .into());
    }
    Ok(())
}
