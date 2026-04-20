use serde::Serialize;
use std::future::Future;
use std::io::Write;

use super::error::AppError;

pub fn format_call_log(prefix: &str, method: &str, args_json: &str) -> String {
    let space = if prefix.is_empty() { "" } else { " " };
    format!("\n\x1b[33m{prefix}{space}{method} with arguments: {args_json}\x1b[0m")
}

pub fn format_return_log(method: &str, return_json: &str) -> String {
    format!("\n\x1b[33mMethod: {method} return: {return_json}\x1b[0m")
}

pub fn format_error_log(method: &str, error_message: &str) -> String {
    format!("\n\x1b[33mMethod: {method} Error: {error_message}\x1b[0m")
}

fn write_stdout(msg: &str) {
    let _ = std::io::stdout().write_all(msg.as_bytes());
}

/// Equivalent of `@Log()` decorator from the original skeleton.
/// Logs the method name, arguments and result (or error).
pub async fn log_method<F, T, A>(
    prefix: &str,
    method: &str,
    args: &A,
    fut: F,
) -> Result<T, AppError>
where
    F: Future<Output = Result<T, AppError>>,
    T: Serialize,
    A: Serialize,
{
    let args_json =
        serde_json::to_string(args).unwrap_or_else(|_| "\"<unserializable>\"".to_string());
    write_stdout(&format_call_log(prefix, method, &args_json));

    match fut.await {
        Ok(result) => {
            let return_json = serde_json::to_string(&result)
                .unwrap_or_else(|_| "\"<unserializable>\"".to_string());
            write_stdout(&format_return_log(method, &return_json));
            Ok(result)
        }
        Err(err) => {
            write_stdout(&format_error_log(method, &err.to_string()));
            Err(err)
        }
    }
}
