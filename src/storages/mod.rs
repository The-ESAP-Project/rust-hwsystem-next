#[macro_use]
mod macros;

use std::sync::Arc;
use tracing::error;

use crate::models::{
    classes::{
        entities::Class,
        requests::{ClassListQuery, CreateClassRequest, UpdateClassRequest},
        responses::ClassListResponse,
    },
    files::entities::File,
    homeworks::{requests::HomeworkListQuery, responses::HomeworkListResponse},
    users::{
        entities::User,
        requests::{CreateUserRequest, UpdateUserRequest, UserListQuery},
        responses::UserListResponse,
    },
};

use crate::errors::{HWSystemError, Result};
use crate::system::app_config::AppConfig;

pub mod backends;
pub mod register;
use register::get_storage_plugin;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    // 用户管理方法
    async fn create_user(&self, user: CreateUserRequest) -> Result<User>;
    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>>;
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn get_user_by_username_or_email(&self, identifier: &str) -> Result<Option<User>>;
    async fn list_users_with_pagination(&self, query: UserListQuery) -> Result<UserListResponse>;
    async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<Option<User>>;
    async fn delete_user(&self, id: i64) -> Result<bool>;
    async fn update_last_login(&self, id: i64) -> Result<bool>;

    // 文件管理方法
    async fn upload_file(
        &self,
        submission_token: &str,
        file_name: &str,
        file_size: &i64,
        file_type: &str,
        user_id: i64,
    ) -> Result<File>;
    async fn get_file_by_token(&self, file_id: String) -> Result<Option<File>>;

    // 班级管理方法
    async fn create_class(&self, class: CreateClassRequest) -> Result<Class>;
    async fn get_class_by_id(&self, class_id: i64) -> Result<Option<Class>>;
    async fn list_classes_with_pagination(
        &self,
        query: ClassListQuery,
    ) -> Result<ClassListResponse>;
    async fn update_class(
        &self,
        class_id: i64,
        update: UpdateClassRequest,
    ) -> Result<Option<Class>>;
    async fn delete_class(&self, class_id: i64) -> Result<bool>;

    // 作业管理方法
    async fn list_homeworks_with_pagination(
        &self,
        query: HomeworkListQuery,
    ) -> Result<HomeworkListResponse>;
}

pub struct StorageFactory;

impl StorageFactory {
    pub async fn create() -> Result<Arc<dyn Storage>> {
        let config = AppConfig::get();
        let backend = &config.database.backend;

        if let Some(ctor) = get_storage_plugin(backend) {
            let storage = ctor().await?;
            Ok(Arc::from(storage))
        } else {
            error!("Failed to create storage backend: {}", backend);
            let available_backends = register::get_storage_plugin_names();
            error!("Available storage backends: {:?}", available_backends);
            Err(HWSystemError::storage_plugin_not_found(format!(
                "Unknown storage backend: {backend}"
            )))
        }
    }
}
