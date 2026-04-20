use axum::body::to_bytes;
use http::{Request, StatusCode};
use rust_skeleton::config::env::load_env;
use rust_skeleton::plugins::healthcheck::routes;
use serde_json::Value;
use tower::ServiceExt;

use crate::common::ENV_LOCK;

#[tokio::test]
async fn healthcheck_responds_ok() {
    let env = {
        let _guard = ENV_LOCK.lock().unwrap();
        load_env()
    };
    let app = routes(&env);
    let url = format!("{}/healthcheck", env.strip_prefix.path);
    let req = Request::builder()
        .uri(url)
        .body(axum::body::Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = to_bytes(resp.into_body(), 1024).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "ok");
}
