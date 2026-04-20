use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::config::Env;

async fn healthcheck() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now(),
    }))
}

pub fn routes(env: &Env) -> Router {
    let path = format!("{}/healthcheck", env.strip_prefix.path);
    Router::new().route(&path, get(healthcheck))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use http::{Request, StatusCode};
    use tower::ServiceExt;

    fn env_for_test() -> Env {
        crate::config::env::load_env()
    }

    #[tokio::test]
    async fn healthcheck_responds_ok() {
        let env = env_for_test();
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
}
