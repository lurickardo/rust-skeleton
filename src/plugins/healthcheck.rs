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
