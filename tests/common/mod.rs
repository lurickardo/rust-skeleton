use async_trait::async_trait;
use mockall::mock;
use std::sync::Mutex;

use rust_skeleton::config::error::AppError;
use rust_skeleton::v1::modules::user::dto::{CreateUserDto, UpdateUserDto};
use rust_skeleton::v1::modules::user::schema::{DeleteMessage, UserResponse};
use rust_skeleton::v1::modules::user::service::UserServiceTrait;

/// Serialized env-var access across tests that manipulate std::env.
pub static ENV_LOCK: Mutex<()> = Mutex::new(());

mock! {
    pub UserService {}

    #[async_trait]
    impl UserServiceTrait for UserService {
        async fn find_by_id(&self, id: String) -> Result<UserResponse, AppError>;
        async fn list_all(&self) -> Result<Vec<UserResponse>, AppError>;
        async fn create(&self, dto: CreateUserDto) -> Result<UserResponse, AppError>;
        async fn update(&self, id: String, dto: UpdateUserDto) -> Result<UserResponse, AppError>;
        async fn remove(&self, id: String) -> Result<DeleteMessage, AppError>;
    }
}
