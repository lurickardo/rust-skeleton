use async_trait::async_trait;
use rand::Rng;

use crate::config::error::{http_exception, AppError};
use crate::config::logger::log_method;

use super::dto::{CreateUserDto, UpdateUserDto};
use super::schema::{DeleteMessage, UserResponse};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserServiceTrait: Send + Sync {
    async fn find_by_id(&self, id: String) -> Result<UserResponse, AppError>;
    async fn list_all(&self) -> Result<Vec<UserResponse>, AppError>;
    async fn create(&self, dto: CreateUserDto) -> Result<UserResponse, AppError>;
    async fn update(&self, id: String, dto: UpdateUserDto) -> Result<UserResponse, AppError>;
    async fn remove(&self, id: String) -> Result<DeleteMessage, AppError>;
}

#[derive(Debug, Default, Clone)]
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn find_by_id(&self, id: String) -> Result<UserResponse, AppError> {
        log_method("", "findById", &vec![id.clone()], async move {
            if id.is_empty() {
                return Err(http_exception("Id user not found.", 404));
            }
            Ok(UserResponse {
                id,
                name: "Jhon Doe".to_string(),
                email: "jhondoe@gmail.com".to_string(),
            })
        })
        .await
    }

    async fn list_all(&self) -> Result<Vec<UserResponse>, AppError> {
        let mut rng = rand::thread_rng();
        Ok(vec![
            UserResponse {
                id: rng.gen_range(0..1000).to_string(),
                name: "Jhon Doe".to_string(),
                email: "jhondoe@gmail.com".to_string(),
            },
            UserResponse {
                id: rng.gen_range(0..1000).to_string(),
                name: "Foo Bar".to_string(),
                email: "foobar@gmail.com".to_string(),
            },
        ])
    }

    async fn create(&self, dto: CreateUserDto) -> Result<UserResponse, AppError> {
        let id = rand::thread_rng().gen_range(0..100).to_string();
        Ok(UserResponse {
            id,
            name: dto.name,
            email: dto.email,
        })
    }

    async fn update(&self, id: String, dto: UpdateUserDto) -> Result<UserResponse, AppError> {
        if id.is_empty() {
            return Err(http_exception("Id user not found.", 404));
        }
        Ok(UserResponse {
            id,
            name: dto.name,
            email: dto.email,
        })
    }

    async fn remove(&self, id: String) -> Result<DeleteMessage, AppError> {
        if id.is_empty() {
            return Err(http_exception("Id user not found.", 404));
        }
        Ok(DeleteMessage {
            message: "User successfully removed".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn find_by_id_returns_user() {
        let svc = UserService::new();
        let result = svc.find_by_id("1".to_string()).await.unwrap();
        assert_eq!(result.id, "1");
        assert_eq!(result.name, "Jhon Doe");
        assert_eq!(result.email, "jhondoe@gmail.com");
    }

    #[tokio::test]
    async fn find_by_id_fails_without_id() {
        let svc = UserService::new();
        let err = svc.find_by_id(String::new()).await.unwrap_err();
        match err {
            AppError::Http {
                status_code,
                message,
            } => {
                assert_eq!(status_code, 404);
                assert_eq!(message, "Id user not found.");
            }
            _ => panic!("expected Http error"),
        }
    }

    #[tokio::test]
    async fn list_all_returns_seed() {
        let svc = UserService::new();
        let list = svc.list_all().await.unwrap();
        assert_eq!(list.len(), 2);
        assert!(!list[0].id.is_empty());
        assert!(!list[0].name.is_empty());
        assert!(!list[0].email.is_empty());
    }

    #[tokio::test]
    async fn create_returns_user() {
        let svc = UserService::new();
        let dto = CreateUserDto {
            name: "foo".into(),
            email: "bar@baz.com".into(),
        };
        let user = svc.create(dto).await.unwrap();
        assert_eq!(user.name, "foo");
        assert_eq!(user.email, "bar@baz.com");
    }

    #[tokio::test]
    async fn update_returns_user() {
        let svc = UserService::new();
        let dto = UpdateUserDto {
            name: "updatedName".into(),
            email: "updated@email.com".into(),
        };
        let user = svc.update("1".to_string(), dto).await.unwrap();
        assert_eq!(user.id, "1");
        assert_eq!(user.name, "updatedName");
        assert_eq!(user.email, "updated@email.com");
    }

    #[tokio::test]
    async fn update_fails_without_id() {
        let svc = UserService::new();
        let dto = UpdateUserDto {
            name: "a".into(),
            email: "a@b.c".into(),
        };
        let err = svc.update(String::new(), dto).await.unwrap_err();
        match err {
            AppError::Http {
                status_code,
                message,
            } => {
                assert_eq!(status_code, 404);
                assert_eq!(message, "Id user not found.");
            }
            _ => panic!("expected Http error"),
        }
    }

    #[tokio::test]
    async fn remove_returns_message() {
        let svc = UserService::new();
        let result = svc.remove("1".to_string()).await.unwrap();
        assert_eq!(result.message, "User successfully removed");
    }

    #[tokio::test]
    async fn remove_fails_without_id() {
        let svc = UserService::new();
        let err = svc.remove(String::new()).await.unwrap_err();
        match err {
            AppError::Http {
                status_code,
                message,
            } => {
                assert_eq!(status_code, 404);
                assert_eq!(message, "Id user not found.");
            }
            _ => panic!("expected Http error"),
        }
    }
}
