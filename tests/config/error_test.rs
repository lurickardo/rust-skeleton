use rust_skeleton::config::error::{AppError, ErrorMessage, http_exception};

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
fn validation_variant_builds_directly() {
    let err = AppError::Validation(vec!["e1".into(), "e2".into()]);
    match err {
        AppError::Validation(msgs) => assert_eq!(msgs, vec!["e1", "e2"]),
        _ => panic!("wrong variant"),
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
fn into_response_uses_correct_status() {
    use axum::response::IntoResponse;
    use http::StatusCode;
    let err = http_exception("not found", 404);
    let resp = err.into_response();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
