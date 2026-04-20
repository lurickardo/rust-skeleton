use axum::body::Body;
use axum::extract::FromRequest;
use http::Request;
use rust_skeleton::config::error::AppError;
use rust_skeleton::plugins::validation::ValidatedJson;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct TestDto {
    #[validate(email)]
    email: String,
}

#[tokio::test]
async fn valid_payload_passes() {
    let req = Request::builder()
        .method("POST")
        .header("content-type", "application/json")
        .uri("/")
        .body(Body::from(r#"{"email":"a@b.com"}"#))
        .unwrap();
    let result: Result<ValidatedJson<TestDto>, _> = ValidatedJson::from_request(req, &()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn invalid_email_returns_validation_error() {
    let req = Request::builder()
        .method("POST")
        .header("content-type", "application/json")
        .uri("/")
        .body(Body::from(r#"{"email":"not-an-email"}"#))
        .unwrap();
    let result: Result<ValidatedJson<TestDto>, _> = ValidatedJson::from_request(req, &()).await;
    match result {
        Err(AppError::Validation(msgs)) => {
            assert!(msgs.iter().any(|m| m.starts_with("email:")));
        }
        _ => panic!("expected validation error"),
    }
}

#[tokio::test]
async fn malformed_body_returns_fastify_like_error() {
    let req = Request::builder()
        .method("POST")
        .header("content-type", "application/json")
        .uri("/")
        .body(Body::from("not json"))
        .unwrap();
    let result: Result<ValidatedJson<TestDto>, _> = ValidatedJson::from_request(req, &()).await;
    assert!(matches!(result, Err(AppError::FastifyLike(_))));
}
