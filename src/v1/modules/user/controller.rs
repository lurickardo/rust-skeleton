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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v1::modules::user::schema::{DeleteMessage, UserResponse};
    use crate::v1::modules::user::service::MockUserServiceTrait;
    use axum::body::{to_bytes, Body};
    use axum::Router;
    use http::Request;
    use mockall::predicate::eq;
    use tower::ServiceExt;

    fn app_with_mock(mock: MockUserServiceTrait) -> Router {
        let shared: SharedUserService = Arc::new(mock);
        crate::v1::modules::user::routes::routes(shared)
    }

    #[tokio::test]
    async fn find_by_id_returns_200() {
        let mut mock = MockUserServiceTrait::new();
        mock.expect_find_by_id()
            .with(eq("1".to_string()))
            .returning(|id| {
                Ok(UserResponse {
                    id,
                    name: "Jhon Doe".into(),
                    email: "jhondoe@gmail.com".into(),
                })
            });
        let app = app_with_mock(mock);
        let req = Request::builder()
            .uri("/v1/user/1")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value =
            serde_json::from_slice(&to_bytes(resp.into_body(), 1024).await.unwrap()).unwrap();
        assert_eq!(body["_id"], "1");
    }

    #[tokio::test]
    async fn list_all_returns_200() {
        let mut mock = MockUserServiceTrait::new();
        mock.expect_list_all().returning(|| {
            Ok(vec![UserResponse {
                id: "1".into(),
                name: "Jhon Doe".into(),
                email: "jhondoe@gmail.com".into(),
            }])
        });
        let app = app_with_mock(mock);
        let req = Request::builder()
            .uri("/v1/user")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_returns_201() {
        let mut mock = MockUserServiceTrait::new();
        mock.expect_create().returning(|dto| {
            Ok(UserResponse {
                id: "1".into(),
                name: dto.name,
                email: dto.email,
            })
        });
        let app = app_with_mock(mock);
        let req = Request::builder()
            .method("POST")
            .uri("/v1/user")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name":"Jhon","email":"jhon@doe.com"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn update_returns_200() {
        let mut mock = MockUserServiceTrait::new();
        mock.expect_update().returning(|id, dto| {
            Ok(UserResponse {
                id,
                name: dto.name,
                email: dto.email,
            })
        });
        let app = app_with_mock(mock);
        let req = Request::builder()
            .method("PUT")
            .uri("/v1/user/1")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name":"Jhon","email":"jhon@doe.com"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn remove_returns_200() {
        let mut mock = MockUserServiceTrait::new();
        mock.expect_remove().returning(|_| {
            Ok(DeleteMessage {
                message: "User deleted".into(),
            })
        });
        let app = app_with_mock(mock);
        let req = Request::builder()
            .method("DELETE")
            .uri("/v1/user/1")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_returns_400_on_invalid_email() {
        let mock = MockUserServiceTrait::new();
        let app = app_with_mock(mock);
        let req = Request::builder()
            .method("POST")
            .uri("/v1/user")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"name":"a","email":"not-email"}"#))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
