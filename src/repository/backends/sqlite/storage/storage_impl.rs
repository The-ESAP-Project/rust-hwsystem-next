use super::SqliteStorage;
use crate::{
    models::{
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
    },
    repository::backends::sqlite::storage::class_users,
};

use super::{classes, file, homeworks, user};
use crate::errors::Result;
use crate::repository::Storage;
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

    /// 作业模块
    async fn list_homeworks_with_pagination(
        &self,
        query: HomeworkListQuery,
    ) -> Result<HomeworkListResponse> {
        homeworks::list_homeworks_with_pagination(self, query).await
    }

    /// 班级模块
    async fn create_class(&self, class: CreateClassRequest) -> Result<Class> {
        classes::create_class(self, class).await
    }

    async fn get_class_by_id(&self, class_id: i64) -> Result<Option<Class>> {
        classes::get_class_by_id(self, class_id).await
    }

    async fn get_class_by_code(&self, invite_code: &str) -> Result<Option<Class>> {
        classes::get_class_by_code(self, invite_code).await
    }

    async fn list_classes_with_pagination(
        &self,
        query: ClassListQuery,
    ) -> Result<ClassListResponse> {
        classes::list_classes_with_pagination(self, query).await
    }

    async fn update_class(
        &self,
        class_id: i64,
        update: UpdateClassRequest,
    ) -> Result<Option<Class>> {
        classes::update_class(self, class_id, update).await
    }

    async fn delete_class(&self, class_id: i64) -> Result<bool> {
        classes::delete_class(self, class_id).await
    }

    /// 班级学生管理方法
    async fn join_class(
        &self,
        user_id: i64,
        class_id: i64,
        role: ClassUserRole,
    ) -> Result<ClassUser> {
        class_users::join_class(self, user_id, class_id, role).await
    }

    async fn leave_class(&self, user_id: i64, class_id: i64) -> Result<bool> {
        class_users::leave_class(self, user_id, class_id).await
    }

    async fn update_class_user(
        &self,
        class_id: i64,
        class_user_id: i64,
        update_data: UpdateClassUserRequest,
    ) -> Result<Option<ClassUser>> {
        class_users::update_class_user(self, class_id, class_user_id, update_data).await
    }

    async fn list_class_users_with_pagination(
        &self,
        class_id: i64,
        query: ClassUserQuery,
    ) -> Result<ClassUserListResponse> {
        class_users::list_class_users_with_pagination(self, class_id, query).await
    }

    async fn list_user_classes_with_pagination(
        &self,
        user_id: i64,
        query: ClassListQuery,
    ) -> Result<ClassListResponse> {
        class_users::list_user_classes_with_pagination(self, user_id, query).await
    }

    async fn get_class_user_by_user_id_and_class_id(
        &self,
        user_id: i64,
        class_id: i64,
    ) -> Result<Option<ClassUser>> {
        class_users::get_class_user_by_user_id_and_class_id(self, user_id, class_id).await
    }

    async fn get_class_and_class_user_by_class_id_and_code(
        &self,
        class_id: i64,
        invite_code: &str,
        user_id: i64,
    ) -> Result<(Option<Class>, Option<ClassUser>)> {
        class_users::get_class_and_class_user_by_class_id_and_code(
            self,
            class_id,
            invite_code,
            user_id,
        )
        .await
    }

    /// 文件模块
    async fn upload_file(
        &self,
        submission_token: &str,
        file_name: &str,
        file_size: &i64,
        file_type: &str,
        user_id: i64,
    ) -> Result<File> {
        // 文件上传逻辑
        file::upload_file(
            self,
            submission_token,
            file_name,
            file_size,
            file_type,
            user_id,
        )
        .await
    }

    async fn get_file_by_token(&self, file_id: &str) -> Result<Option<File>> {
        // 获取文件逻辑
        file::get_file_by_token(self, file_id).await
    }
}
