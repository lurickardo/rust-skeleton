use std::sync::Arc;

use axum::body::{to_bytes, Body};
use axum::http::StatusCode;
use http::Request;
use mockall::predicate::eq;
use rust_skeleton::v1::modules::user::controller::SharedUserService;
use rust_skeleton::v1::modules::user::routes::routes;
use rust_skeleton::v1::modules::user::schema::{DeleteMessage, UserResponse};
use serde_json::Value;
use tower::ServiceExt;

use crate::common::MockUserService;

fn app_with_mock(mock: MockUserService) -> axum::Router {
    let shared: SharedUserService = Arc::new(mock);
    routes(shared)
}

#[tokio::test]
async fn find_by_id_returns_200() {
    let mut mock = MockUserService::new();
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
    let mut mock = MockUserService::new();
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
    let mut mock = MockUserService::new();
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
    let mut mock = MockUserService::new();
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
    let mut mock = MockUserService::new();
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
    let mock = MockUserService::new();
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
