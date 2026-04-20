use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::config::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {
    #[validate(length(min = 1, message = "Required"))]
    pub name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDto {
    #[validate(length(min = 1, message = "Required"))]
    pub name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
}

fn collect_errors(errors: validator::ValidationErrors) -> Vec<String> {
    errors
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
        .collect()
}

pub fn transform_create_user_dto(data: CreateUserDto) -> Result<CreateUserDto, AppError> {
    data.validate()
        .map_err(|e| AppError::Validation(collect_errors(e)))?;
    Ok(data)
}

pub fn transform_update_user_dto(data: UpdateUserDto) -> Result<UpdateUserDto, AppError> {
    data.validate()
        .map_err(|e| AppError::Validation(collect_errors(e)))?;
    Ok(data)
}
