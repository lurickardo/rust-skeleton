use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::config::error::AppError;
use crate::plugins::validation::ValidatedJson;

use super::dto::{
    transform_create_user_dto, transform_update_user_dto, CreateUserDto, UpdateUserDto,
};
use super::service::UserServiceTrait;

pub type SharedUserService = Arc<dyn UserServiceTrait>;

#[utoipa::path(
    get,
    path = "/v1/user/{id}",
    tag = "v1",
    params(("id" = String, Path, description = "User id")),
    responses((status = 200, description = "Find data of user by id", body = crate::v1::modules::user::schema::UserResponse))
)]
pub async fn find_by_id(
    State(service): State<SharedUserService>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let user = service.find_by_id(id).await?;
    Ok((StatusCode::OK, Json(json!(user))))
}

#[utoipa::path(
    get,
    path = "/v1/user",
    tag = "v1",
    responses((status = 200, description = "Find data of all users", body = [crate::v1::modules::user::schema::UserResponse]))
)]
pub async fn list_all(
    State(service): State<SharedUserService>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let users = service.list_all().await?;
    Ok((StatusCode::OK, Json(json!(users))))
}

#[utoipa::path(
    post,
    path = "/v1/user",
    tag = "v1",
    request_body = CreateUserDto,
    responses((status = 201, description = "Create user", body = crate::v1::modules::user::schema::UserResponse))
)]
pub async fn create(
    State(service): State<SharedUserService>,
    ValidatedJson(body): ValidatedJson<CreateUserDto>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let dto = transform_create_user_dto(body)?;
    let user = service.create(dto).await?;
    Ok((StatusCode::CREATED, Json(json!(user))))
}

#[utoipa::path(
    put,
    path = "/v1/user/{id}",
    tag = "v1",
    params(("id" = String, Path, description = "User id")),
    request_body = UpdateUserDto,
    responses((status = 200, description = "Update user", body = crate::v1::modules::user::schema::UserResponse))
)]
pub async fn update(
    State(service): State<SharedUserService>,
    Path(id): Path<String>,
    ValidatedJson(body): ValidatedJson<UpdateUserDto>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let dto = transform_update_user_dto(body)?;
    let user = service.update(id, dto).await?;
    Ok((StatusCode::OK, Json(json!(user))))
}

#[utoipa::path(
    delete,
    path = "/v1/user/{id}",
    tag = "v1",
    params(("id" = String, Path, description = "User id")),
    responses((status = 200, description = "Remove user", body = crate::v1::modules::user::schema::DeleteMessage))
)]
pub async fn remove(
    State(service): State<SharedUserService>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    let msg = service.remove(id).await?;
    Ok((StatusCode::OK, Json(json!(msg))))
}
