use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid request {0} input")]
    FastifyLike(String),

    #[error("Validation error")]
    Validation(Vec<String>),

    #[error("{message}")]
    Http { message: String, status_code: u16 },

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ErrorMessage {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub message: ErrorMessage,
    pub timestamp: DateTime<Utc>,
}

pub fn http_exception(message: impl Into<String>, status_code: u16) -> AppError {
    AppError::Http {
        message: message.into(),
        status_code,
    }
}

pub fn http_exception_many(messages: Vec<String>, status_code: u16) -> AppError {
    AppError::Validation(messages).with_status(status_code)
}

impl AppError {
    fn with_status(self, _status: u16) -> Self {
        self
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::FastifyLike(_) => StatusCode::BAD_REQUEST,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Http { status_code, .. } => {
                StatusCode::from_u16(*status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
            AppError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn into_body(self) -> ErrorResponse {
        let status = self.status_code();
        let (message, status_u16) = match self {
            AppError::FastifyLike(ctx) => {
                let validation_context = if ctx.is_empty() {
                    String::new()
                } else {
                    format!("{ctx} ")
                };
                (
                    ErrorMessage::Single(format!("Invalid request {validation_context}input")),
                    status.as_u16(),
                )
            }
            AppError::Validation(msgs) => (ErrorMessage::Many(msgs), status.as_u16()),
            AppError::Http {
                message,
                status_code,
            } => (ErrorMessage::Single(message), status_code),
            AppError::Unexpected(inner) => {
                print_unexpected_error(&inner);
                (
                    ErrorMessage::Single("Internal Server Error".to_string()),
                    status.as_u16(),
                )
            }
        };

        ErrorResponse {
            status_code: status_u16,
            message,
            timestamp: Utc::now(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = self.into_body();
        (status, Json(body)).into_response()
    }
}

fn print_unexpected_error(detail: &str) {
    let _ = std::io::Write::write_all(
        &mut std::io::stdout(),
        format!(
            "\n\n\x1b[41m--- UNEXPECTED ERROR --- \x1b[0m\n {detail}\n\x1b[41m--- END UNEXPECTED ERROR --- \x1b[0m\n\n\n"
        )
        .as_bytes(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_status(err: &AppError) -> u16 {
        err.status_code().as_u16()
    }

    #[test]
    fn fastify_like_with_context() {
        let err = AppError::FastifyLike("Test".to_string());
        assert_eq!(extract_status(&err), 400);
        let body = err.into_body();
        match body.message {
            ErrorMessage::Single(m) => assert_eq!(m, "Invalid request Test input"),
            _ => panic!("expected single"),
        }
        assert_eq!(body.status_code, 400);
    }

    #[test]
    fn fastify_like_without_context() {
        let err = AppError::FastifyLike(String::new());
        let body = err.into_body();
        match body.message {
            ErrorMessage::Single(m) => assert_eq!(m, "Invalid request input"),
            _ => panic!("expected single"),
        }
    }

    #[test]
    fn validation_returns_many_messages() {
        let err = AppError::Validation(vec!["field: Invalid input".to_string()]);
        assert_eq!(extract_status(&err), 400);
        let body = err.into_body();
        match body.message {
            ErrorMessage::Many(ms) => assert_eq!(ms, vec!["field: Invalid input"]),
            _ => panic!("expected many"),
        }
    }

    #[test]
    fn flow_error_preserves_status_and_message() {
        let err = http_exception("Flow error", 404);
        assert_eq!(extract_status(&err), 404);
        let body = err.into_body();
        match body.message {
            ErrorMessage::Single(m) => assert_eq!(m, "Flow error"),
            _ => panic!("expected single"),
        }
        assert_eq!(body.status_code, 404);
    }

    #[test]
    fn unexpected_returns_500() {
        let err = AppError::Unexpected("boom".to_string());
        assert_eq!(extract_status(&err), 500);
        let body = err.into_body();
        match body.message {
            ErrorMessage::Single(m) => assert_eq!(m, "Internal Server Error"),
            _ => panic!("expected single"),
        }
        assert_eq!(body.status_code, 500);
    }

    #[test]
    fn unexpected_with_empty_detail_still_returns_500() {
        let err = AppError::Unexpected(String::new());
        let body = err.into_body();
        assert_eq!(body.status_code, 500);
    }

    #[test]
    fn http_exception_builds_correct_variant() {
        let err = http_exception("Some error", 400);
        match err {
            AppError::Http {
                message,
                status_code,
            } => {
                assert_eq!(message, "Some error");
                assert_eq!(status_code, 400);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn http_exception_many_builds_validation_variant() {
        let err = http_exception_many(vec!["e1".into(), "e2".into()], 500);
        match err {
            AppError::Validation(msgs) => assert_eq!(msgs, vec!["e1", "e2"]),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn into_response_uses_correct_status() {
        use http::StatusCode;
        let err = http_exception("not found", 404);
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
