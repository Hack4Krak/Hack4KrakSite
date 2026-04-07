use crate::entities::sea_orm_active_enums::UserRoles;
use crate::entities::users;
use crate::models::event_config::{EventConfig, EventStage, EventStageType};
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

#[derive(Clone, Copy, PartialEq)]
enum StageCheck {
    Before,
    During,
    After,
}

#[derive(Clone)]
enum StageIdentifier {
    ByName(String),
    ByType(EventStageType),
}

impl StageIdentifier {
    fn get_stage(&self, event_config: &EventConfig) -> Result<EventStage, Error> {
        let stage = match self {
            StageIdentifier::ByName(name) => event_config.stage_by_name(name).cloned(),
            StageIdentifier::ByType(stage_type) => event_config.stage_by_type(stage_type).cloned(),
        };

        stage.ok_or(Error::FailedToParseStage {
            stage_identifier: match self {
                StageIdentifier::ByName(name) => format!("name: {}", name),
                StageIdentifier::ByType(stage_type) => format!("type: {:?}", stage_type),
            },
        })
    }
}

#[derive(Clone)]
enum EventMiddlewareMode {
    SingleStage {
        identifier: StageIdentifier,
        check: StageCheck,
    },
    BetweenStages {
        starting_stage: StageIdentifier,
        ending_stage: StageIdentifier,
    },
}

/// Restricts access to endpoints based on event stage status
pub struct EventMiddleware {
    mode: EventMiddlewareMode,
}

impl EventMiddleware {
    pub fn allow_only_during_event() -> Self {
        Self {
            mode: EventMiddlewareMode::BetweenStages {
                starting_stage: StageIdentifier::ByType(EventStageType::EventStart),
                ending_stage: StageIdentifier::ByType(EventStageType::EventEnd),
            },
        }
    }

    pub fn allow_only_after_event() -> Self {
        Self {
            mode: EventMiddlewareMode::SingleStage {
                identifier: StageIdentifier::ByType(EventStageType::EventEnd),
                check: StageCheck::After,
            },
        }
    }

    pub fn disallow_before_event() -> Self {
        Self {
            mode: EventMiddlewareMode::SingleStage {
                identifier: StageIdentifier::ByType(EventStageType::EventStart),
                check: StageCheck::After,
            },
        }
    }

    pub fn before_stage(stage_name: impl Into<String>) -> Self {
        Self {
            mode: EventMiddlewareMode::SingleStage {
                identifier: StageIdentifier::ByName(stage_name.into()),
                check: StageCheck::Before,
            },
        }
    }

    pub fn during_stage(stage_name: impl Into<String>) -> Self {
        Self {
            mode: EventMiddlewareMode::SingleStage {
                identifier: StageIdentifier::ByName(stage_name.into()),
                check: StageCheck::During,
            },
        }
    }

    pub fn after_stage(stage_name: impl Into<String>) -> Self {
        Self {
            mode: EventMiddlewareMode::SingleStage {
                identifier: StageIdentifier::ByName(stage_name.into()),
                check: StageCheck::After,
            },
        }
    }

    pub fn between_stage(
        starting_stage: impl Into<String>,
        ending_stage: impl Into<String>,
    ) -> Self {
        Self {
            mode: EventMiddlewareMode::BetweenStages {
                starting_stage: StageIdentifier::ByName(starting_stage.into()),
                ending_stage: StageIdentifier::ByName(ending_stage.into()),
            },
        }
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
            mode: self.mode.clone(),
        }))
    }
}

pub struct EventMiddlewareService<S> {
    service: Rc<S>,
    mode: EventMiddlewareMode,
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
        let mode = self.mode.clone();

        Box::pin(async move {
            let response = Self::verify_request(&mut request, &mode).await;
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
        mode: &EventMiddlewareMode,
    ) -> Result<(), Error> {
        if let Some(user) = request.extensions().get::<users::Model>()
            && user.roles >= UserRoles::Admin
        {
            return Ok(());
        }

        let task_manager = &request
            .app_data::<web::Data<AppState>>()
            .ok_or(Error::Unauthorized)?
            .task_manager;

        let event_config = task_manager.event_config.read().await;
        let now = Utc::now();

        match mode {
            EventMiddlewareMode::BetweenStages {
                starting_stage,
                ending_stage,
            } => {
                let starting_stage = starting_stage.get_stage(&event_config)?;
                let end_date = ending_stage.get_stage(&event_config)?.start_date;

                let start_date = starting_stage.end_date.unwrap_or(starting_stage.start_date);

                if now < start_date {
                    return Err(Error::AccessBeforeStage {
                        stage_start_date: starting_stage.start_date.to_rfc3339(),
                    });
                }

                if now >= end_date {
                    return Err(Error::AccessAfterStage {
                        stage_end_date: end_date.to_rfc3339(),
                    });
                }

                Ok(())
            }
            EventMiddlewareMode::SingleStage { identifier, check } => {
                let stage = identifier.get_stage(&event_config)?;
                let start = stage.start_date;
                let end = stage.end_date;

                let before_start = now < start;

                if before_start && (check == &StageCheck::During || check == &StageCheck::After) {
                    return Err(Error::AccessBeforeStage {
                        stage_start_date: start.to_rfc3339(),
                    });
                } else if !before_start && check == &StageCheck::Before {
                    return Err(Error::AccessDuringStage);
                }

                let Some(end) = end else {
                    return Ok(());
                };

                let before_end = end > now;

                if !before_end && check == &StageCheck::During {
                    return Err(Error::AccessAfterStage {
                        stage_end_date: end.to_rfc3339(),
                    });
                } else if before_end && check == &StageCheck::After {
                    return Err(Error::AccessDuringStage);
                }

                Ok(())
            }
        }
    }
}
