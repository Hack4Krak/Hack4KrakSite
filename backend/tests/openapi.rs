use hack4krak_backend::setup_actix_app;
use tokio::fs::read_to_string;

#[actix_web::test]
async fn test_openapi() {
    let (_, api) = setup_actix_app().split_for_parts();
    let openapi = api.to_json().unwrap();
    let generated_openapi = read_to_string("../frontend/openapi/api/openapi.json")
        .await
        .unwrap();
    assert_eq!(openapi, generated_openapi);
}
