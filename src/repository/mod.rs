#[macro_use]
mod macros;

use crate::models::users::entities::UserRole;
use std::sync::Arc;
use tracing::error;

use crate::models::{
    class_users::{
        entities::{ClassUser, ClassUserRole},
        requests::{ClassUserQuery, UpdateClassUserRequest},
        responses::ClassUserListResponse,
    },
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
    /// 用户管理方法
    // 创建用户
    async fn create_user(&self, user: CreateUserRequest) -> Result<User>;
    // 通过ID获取用户信息
    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>>;
    // 通过用户名获取用户信息
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>>;
    // 通过邮箱获取用户信息
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>>;
    // 通过用户名或邮箱获取用户信息
    async fn get_user_by_username_or_email(&self, identifier: &str) -> Result<Option<User>>;
    // 列出用户
    async fn list_users_with_pagination(&self, query: UserListQuery) -> Result<UserListResponse>;
    // 更新用户信息
    async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<Option<User>>;
    // 删除用户
    async fn delete_user(&self, id: i64) -> Result<bool>;
    // 更新用户最后登录时间
    async fn update_last_login(&self, id: i64) -> Result<bool>;

    /// 文件管理方法
    // 上传文件
    async fn upload_file(
        &self,
        submission_token: &str,
        file_name: &str,
        file_size: &i64,
        file_type: &str,
        user_id: i64,
    ) -> Result<File>;
    // 通过唯一 token 获取文件信息
    async fn get_file_by_token(&self, file_id: &str) -> Result<Option<File>>;

    /// 班级管理方法
    // 创建班级
    async fn create_class(&self, class: CreateClassRequest) -> Result<Class>;
    // 通过ID获取班级信息
    async fn get_class_by_id(&self, class_id: i64) -> Result<Option<Class>>;
    // 通过邀请码获取班级信息
    async fn get_class_by_code(&self, invite_code: &str) -> Result<Option<Class>>;
    // 列出班级
    async fn list_classes_with_pagination(
        &self,
        query: ClassListQuery,
    ) -> Result<ClassListResponse>;
    // 更新班级信息
    async fn update_class(
        &self,
        class_id: i64,
        update: UpdateClassRequest,
    ) -> Result<Option<Class>>;
    // 删除班级
    async fn delete_class(&self, class_id: i64) -> Result<bool>;

    /// 班级学生管理方法
    // 学生加入班级，通过邀请码并指定角色
    async fn join_class(
        &self,
        user_id: i64,
        class_id: i64,
        role: ClassUserRole,
    ) -> Result<ClassUser>;
    // 学生离开/踢出班级
    async fn leave_class(&self, user_id: i64, class_id: i64) -> Result<bool>;
    // 更新班级用户信息
    async fn update_class_user(
        &self,
        class_id: i64,
        class_user_id: i64,
        update_data: UpdateClassUserRequest,
    ) -> Result<Option<ClassUser>>;
    // 列出班级用户
    async fn list_class_users_with_pagination( 
        &self,
        class_id: i64,
        query: ClassUserQuery,
    ) -> Result<ClassUserListResponse>;
    // 列出用户所在的班级
    async fn list_user_classes_with_pagination(
        &self,
        user_id: i64,
        query: ClassListQuery,
    ) -> Result<ClassListResponse>;
    // 获取用户在班级中的信息
    async fn get_class_user_by_user_id_and_class_id(
        &self,
        user_id: i64,
        class_id: i64,
    ) -> Result<Option<ClassUser>>;
    // 根据班级ID和邀请码获取班级及用户信息
    async fn get_class_and_class_user_by_class_id_and_code(
        &self,
        class_id: i64,
        invite_code: &str,
        user_id: i64,
    ) -> Result<(Option<Class>, Option<ClassUser>)>;

    // 作业管理方法
    async fn list_homeworks_with_pagination(
        &self,
        user_id: i64,
        user_role: UserRole,
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
