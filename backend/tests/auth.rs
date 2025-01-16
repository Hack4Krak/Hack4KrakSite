use actix_web::web::Data;
use actix_web::{test, App};
use hack4krak_backend::models::entities::users;
use hack4krak_backend::routes;
use hack4krak_backend::utils::app_state::AppState;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use utoipa::gen::serde_json::json;
use utoipa_actix_web::scope;

#[actix_web::test]
async fn auth_flow() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            //     // First query result
            Vec::<users::Model>::new(),
            vec![users::Model {
                username: "".to_string(),
                email: "".to_string(),
                created_at: Default::default(),
                team_name: None,
                permissions: None,
                leads: None,
                password: None,
            }],
        ])
        .append_exec_results([MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let data = Data::new(AppState { database: db });

    // todo: is init_serive required
    let app = test::init_service(
        App::new()
            .app_data(data.clone())
            .service(scope("/auth").configure(routes::auth::config)),
    )
    .await;

    let register_payload = json!({
        "email": "test@example.com",
        "name": "test_user",
        "password": "password123"
    });

    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&register_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
