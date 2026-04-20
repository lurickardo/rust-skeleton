use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::config::error::AppError;

/// Extractor equivalent to the Fastify AJV `schemaCompiler` + Zod `parse`.
/// Deserializes the JSON body via serde and then validates via `validator`.
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| AppError::FastifyLike(String::new()))?;

        value.validate().map_err(|e| {
            let msgs: Vec<String> = e
                .field_errors()
                .iter()
                .flat_map(|(field, errs)| {
                    errs.iter().map(move |err| {
                        let detail = err
                            .message
                            .clone()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| err.code.to_string());
                        format!("{field}: {detail}")
                    })
                })
                .collect();
            AppError::Validation(msgs)
        })?;

        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http::Request;
    use serde::Deserialize;

    #[derive(Deserialize, Validate)]
    struct TestDto {
        #[validate(email)]
        email: String,
    }

    #[tokio::test]
    async fn valid_payload_passes() {
        let req = Request::builder()
            .method("POST")
            .header("content-type", "application/json")
            .uri("/")
            .body(Body::from(r#"{"email":"a@b.com"}"#))
            .unwrap();
        let result: Result<ValidatedJson<TestDto>, _> = ValidatedJson::from_request(req, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn invalid_email_returns_validation_error() {
        let req = Request::builder()
            .method("POST")
            .header("content-type", "application/json")
            .uri("/")
            .body(Body::from(r#"{"email":"not-an-email"}"#))
            .unwrap();
        let result: Result<ValidatedJson<TestDto>, _> = ValidatedJson::from_request(req, &()).await;
        match result {
            Err(AppError::Validation(msgs)) => {
                assert!(msgs.iter().any(|m| m.starts_with("email:")));
            }
            _ => panic!("expected validation error"),
        }
    }

    #[tokio::test]
    async fn malformed_body_returns_fastify_like_error() {
        let req = Request::builder()
            .method("POST")
            .header("content-type", "application/json")
            .uri("/")
            .body(Body::from("not json"))
            .unwrap();
        let result: Result<ValidatedJson<TestDto>, _> = ValidatedJson::from_request(req, &()).await;
        assert!(matches!(result, Err(AppError::FastifyLike(_))));
    }
}
