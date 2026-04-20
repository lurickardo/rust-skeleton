use axum::{extract::Request, middleware::Next, response::Response};

use crate::config::error::AppError;

/// Middleware equivalent of `UserMiddleware.findById` — passes the request through.
pub async fn find_by_id(request: Request, next: Next) -> Result<Response, AppError> {
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, routing::get, Router};
    use http::{Request as HttpRequest, StatusCode};
    use tower::ServiceExt;

    async fn handler() -> &'static str {
        "ok"
    }

    #[tokio::test]
    async fn middleware_calls_next_without_error() {
        let app = Router::new()
            .route("/", get(handler))
            .layer(axum::middleware::from_fn(find_by_id));
        let req = HttpRequest::builder().uri("/").body(Body::empty()).unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
