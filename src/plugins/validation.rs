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
