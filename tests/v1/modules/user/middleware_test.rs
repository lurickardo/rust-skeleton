use axum::{body::Body, routing::get, Router};
use http::{Request, StatusCode};
use rust_skeleton::v1::modules::user::middleware::find_by_id;
use tower::ServiceExt;

async fn handler() -> &'static str {
    "ok"
}

#[tokio::test]
async fn middleware_calls_next_without_error() {
    let app = Router::new()
        .route("/", get(handler))
        .layer(axum::middleware::from_fn(find_by_id));
    let req = Request::builder().uri("/").body(Body::empty()).unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
