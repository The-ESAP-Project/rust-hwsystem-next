use super::SqliteStorage;
use crate::api_models::users::{
    entities::User,
    requests::{CreateUserRequest, UpdateUserRequest, UserListQuery},
    responses::UserListResponse,
};

use super::user;
use crate::errors::Result;
use crate::storages::Storage;
use async_trait::async_trait;

#[async_trait]
impl Storage for SqliteStorage {
    /// 用户模块
    async fn create_user(&self, user: CreateUserRequest) -> Result<User> {
        user::create_user(self, user).await
    }

    async fn get_user_by_id(&self, id: i64) -> Result<Option<User>> {
        user::get_user_by_id(self, id).await
    }

    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        user::get_user_by_username(self, username).await
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        user::get_user_by_email(self, email).await
    }

    async fn get_user_by_username_or_email(&self, identifier: &str) -> Result<Option<User>> {
        user::get_user_by_username_or_email(self, identifier).await
    }

    async fn list_users_with_pagination(&self, query: UserListQuery) -> Result<UserListResponse> {
        user::list_users_with_pagination(self, query).await
    }

    async fn update_last_login(&self, id: i64) -> Result<bool> {
        user::update_last_login(self, id).await
    }

    async fn update_user(&self, id: i64, update: UpdateUserRequest) -> Result<Option<User>> {
        user::update_user(self, id, update).await
    }

    async fn delete_user(&self, id: i64) -> Result<bool> {
        user::delete_user(self, id).await
    }
}
