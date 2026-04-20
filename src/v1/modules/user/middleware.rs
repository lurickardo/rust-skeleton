use axum::{extract::Request, middleware::Next, response::Response};

use crate::config::error::AppError;

/// Middleware equivalent of `UserMiddleware.findById` — passes the request through.
pub async fn find_by_id(request: Request, next: Next) -> Result<Response, AppError> {
    Ok(next.run(request).await)
}
