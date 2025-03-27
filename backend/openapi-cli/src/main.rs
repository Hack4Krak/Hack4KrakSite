use hack4krak_backend::setup_actix_app;
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() {
    let (_, api) = setup_actix_app(false).split_for_parts();
    let openapi = api.to_json().unwrap();
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 && args[1] == "write" {
        write_openapi(openapi).await;
    } else {
        verify_openapi(openapi).await;
    }
}

async fn verify_openapi(openapi: String) {
    let generated_openapi = read_to_string("../frontend/openapi/api/openapi.json")
        .await
        .unwrap();
    assert_eq!(openapi, generated_openapi, "Your openapi json is...");
    println!("OpenAPI specification is correct!");
}

async fn write_openapi(openapi: String) {
    tokio::fs::write("../frontend/openapi/api/openapi.json", openapi)
        .await
        .unwrap();
    println!("OpenAPI specification is written to frontend/openapi/api/openapi.json");
}
