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

impl AppError {
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
