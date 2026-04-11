use actix_web::web::Data;
use actix_web::{App, HttpResponse, test, web};
use chrono::{DateTime, Duration, FixedOffset, Utc};
use hack4krak_backend::middlewares::event::EventMiddleware;
use hack4krak_backend::models::event_config::{EventStage, EventStageType};
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::utils::app_state::AppState;
use sea_orm::Database;

async fn build_middleware_app(
    tm: TaskManager,
    middleware: EventMiddleware,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
> {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    let mut app_state = AppState::with_database(db);
    app_state.task_manager = tm;

    test::init_service(
        App::new().app_data(Data::new(app_state)).service(
            web::scope("/test")
                .wrap(middleware)
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await
}

fn past(days: i64) -> DateTime<FixedOffset> {
    DateTime::from(Utc::now() - Duration::days(days))
}

fn future(days: i64) -> DateTime<FixedOffset> {
    DateTime::from(Utc::now() + Duration::days(days))
}

fn event_stages(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> Vec<EventStage> {
    vec![
        EventStage {
            name: "Event Start".to_string(),
            stage_type: EventStageType::EventStart,
            start_date: start,
            end_date: None,
        },
        EventStage {
            name: "Event End".to_string(),
            stage_type: EventStageType::EventEnd,
            start_date: end,
            end_date: None,
        },
    ]
}

async fn task_manager_with_stages(stages: Vec<EventStage>) -> TaskManager {
    let tm = TaskManager::default();
    tm.event_config.write().await.stages = stages;
    tm
}

#[actix_web::test]
async fn disallow_before_event_blocks_pre_event() {
    let tm = task_manager_with_stages(event_stages(future(1), future(2))).await;
    let app = build_middleware_app(tm, EventMiddleware::disallow_before_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn disallow_before_event_allows_during() {
    let tm = task_manager_with_stages(event_stages(past(1), future(1))).await;
    let app = build_middleware_app(tm, EventMiddleware::disallow_before_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn disallow_before_event_allows_after() {
    let tm = task_manager_with_stages(event_stages(past(2), past(1))).await;
    let app = build_middleware_app(tm, EventMiddleware::disallow_before_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn allow_only_during_event_blocks_before() {
    let tm = task_manager_with_stages(event_stages(future(1), future(2))).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_during_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn allow_only_during_event_allows_during() {
    let tm = task_manager_with_stages(event_stages(past(1), future(1))).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_during_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn allow_only_during_event_blocks_after() {
    let tm = task_manager_with_stages(event_stages(past(2), past(1))).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_during_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 410);
}

#[actix_web::test]
async fn allow_only_after_event_blocks_before() {
    let tm = task_manager_with_stages(event_stages(future(1), future(2))).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_after_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn allow_only_after_event_blocks_during() {
    let tm = task_manager_with_stages(event_stages(past(1), future(1))).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_after_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn allow_only_after_event_allows_after() {
    let tm = task_manager_with_stages(event_stages(past(2), past(1))).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_after_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn before_stage_allows_before() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: future(1),
        end_date: Some(future(2)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::before_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn before_stage_blocks_during() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(1),
        end_date: Some(future(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::before_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn during_stage_allows_during() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(1),
        end_date: Some(future(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::during_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn during_stage_blocks_before() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: future(1),
        end_date: Some(future(2)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::during_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn during_stage_blocks_after() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(2),
        end_date: Some(past(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::during_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 410);
}

#[actix_web::test]
async fn after_stage_allows_after() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(2),
        end_date: Some(past(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::after_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn after_stage_blocks_during() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(1),
        end_date: Some(future(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::after_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn between_stages_allows_between() {
    let stages = vec![
        EventStage {
            name: "Start".to_string(),
            stage_type: EventStageType::Informative,
            start_date: past(2),
            end_date: None,
        },
        EventStage {
            name: "End".to_string(),
            stage_type: EventStageType::Informative,
            start_date: future(2),
            end_date: None,
        },
    ];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::between_stage("Start", "End")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn between_stages_blocks_before() {
    let stages = vec![
        EventStage {
            name: "Start".to_string(),
            stage_type: EventStageType::Informative,
            start_date: future(1),
            end_date: None,
        },
        EventStage {
            name: "End".to_string(),
            stage_type: EventStageType::Informative,
            start_date: future(2),
            end_date: None,
        },
    ];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::between_stage("Start", "End")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn between_stages_blocks_after() {
    let stages = vec![
        EventStage {
            name: "Start".to_string(),
            stage_type: EventStageType::Informative,
            start_date: past(2),
            end_date: None,
        },
        EventStage {
            name: "End".to_string(),
            stage_type: EventStageType::Informative,
            start_date: past(1),
            end_date: None,
        },
    ];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::between_stage("Start", "End")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 410);
}

#[actix_web::test]
async fn default_task_manager_event_started() {
    let tm = TaskManager::default();
    let app = build_middleware_app(tm, EventMiddleware::disallow_before_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn nonexistent_stage_name_returns_500() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(1),
        end_date: Some(future(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::during_stage("NonExistent")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 500);
}

#[actix_web::test]
async fn after_stage_blocks_before_start() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: future(1),
        end_date: Some(future(2)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::after_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn before_stage_blocks_after_end() {
    let stages = vec![EventStage {
        name: "Registration".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(2),
        end_date: Some(past(1)),
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::before_stage("Registration")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn during_stage_without_end_date_allows_after_start() {
    let stages = vec![EventStage {
        name: "OpenEnded".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(1),
        end_date: None,
    }];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::during_stage("OpenEnded")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn between_stages_uses_end_date_of_starting_stage() {
    let stages = vec![
        EventStage {
            name: "Start".to_string(),
            stage_type: EventStageType::Informative,
            start_date: past(3),
            end_date: Some(future(1)),
        },
        EventStage {
            name: "End".to_string(),
            stage_type: EventStageType::Informative,
            start_date: future(2),
            end_date: None,
        },
    ];
    let tm = task_manager_with_stages(stages).await;
    let app = build_middleware_app(tm, EventMiddleware::between_stage("Start", "End")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    // now is between start.end_date (future(1)) and end.start_date (future(2))
    // but start.end_date = future(1) and now < future(1), so now < start_date (which is end_date of starting stage)
    // This means AccessBeforeStage → 403
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn between_stages_nonexistent_ending_stage_returns_500() {
    let stages = vec![EventStage {
        name: "Start".to_string(),
        stage_type: EventStageType::Informative,
        start_date: past(1),
        end_date: None,
    }];
    let tm = task_manager_with_stages(stages).await;
    let app =
        build_middleware_app(tm, EventMiddleware::between_stage("Start", "NonExistent")).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 500);
}

#[actix_web::test]
async fn allow_only_during_event_with_empty_stages() {
    let tm = task_manager_with_stages(vec![]).await;
    let app = build_middleware_app(tm, EventMiddleware::allow_only_during_event()).await;

    let request = test::TestRequest::get().uri("/test").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 500);
}
