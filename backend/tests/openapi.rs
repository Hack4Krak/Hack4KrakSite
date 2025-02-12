use hack4krak_backend::setup_actix_app;
use std::fs::read_to_string;

#[test]
fn test_openapi() {
    let (_, api) = setup_actix_app().split_for_parts();
    let openapi = api.to_json().unwrap();
    let generated_openapi = read_to_string("../frontend/openapi/api/openapi.json").unwrap();
    assert_eq!(openapi, generated_openapi);
}
