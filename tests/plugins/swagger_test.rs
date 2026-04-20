use rust_skeleton::plugins::swagger::ApiDoc;
use utoipa::OpenApi;

#[test]
fn openapi_document_is_generated() {
    let doc = ApiDoc::openapi();
    let json = serde_json::to_string(&doc).unwrap();
    assert!(json.contains("/v1/user"));
}
