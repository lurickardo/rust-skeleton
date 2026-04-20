use async_trait::async_trait;
use rand::Rng;

use crate::config::error::{http_exception, AppError};
use crate::config::logger::log_method;

use super::dto::{CreateUserDto, UpdateUserDto};
use super::schema::{DeleteMessage, UserResponse};

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
