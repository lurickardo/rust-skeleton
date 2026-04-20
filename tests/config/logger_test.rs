use rust_skeleton::config::error::{AppError, http_exception};
use rust_skeleton::config::logger::{
    format_call_log, format_error_log, format_return_log, log_method,
};

#[test]
fn formats_call_log_with_prefix() {
    let out = format_call_log("Calling testMethod", "testMethod", "[\"World\"]");
    assert!(out.contains("Calling testMethod testMethod with arguments: [\"World\"]"));
    assert!(out.contains("\x1b[33m"));
}

#[test]
fn formats_call_log_without_prefix() {
    let out = format_call_log("", "testMethod", "[]");
    assert!(out.contains("testMethod with arguments: []"));
}

#[test]
fn formats_return_log() {
    let out = format_return_log("testMethod", "\"Hello, World\"");
    assert!(out.contains("Method: testMethod return: \"Hello, World\""));
}

#[test]
fn formats_error_log() {
    let out = format_error_log("testMethod", "Test error");
    assert!(out.contains("Method: testMethod Error: Test error"));
}

#[tokio::test]
async fn log_method_returns_success_value() {
    let args: Vec<String> = vec!["World".into()];
    let result = log_method("Calling testMethod", "testMethod", &args, async {
        Ok::<_, AppError>("Hello, World".to_string())
    })
    .await;
    assert_eq!(result.unwrap(), "Hello, World");
}

#[tokio::test]
async fn log_method_propagates_error() {
    let args: Vec<String> = vec![];
    let result: Result<String, AppError> = log_method("", "testMethod", &args, async {
        Err(http_exception("Test error", 400))
    })
    .await;
    assert!(result.is_err());
}
