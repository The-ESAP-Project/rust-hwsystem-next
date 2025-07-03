use actix_web::{HttpResponse, Result as ActixResult};
use std::sync::Arc;

use crate::api_models::{
    ApiResponse, CreateUserRequest, UpdateUserRequest, UserListQuery, UserQueryParams,
};
use crate::storages::Storage;

// 用户服务
pub struct UserService {
    storage: Arc<dyn Storage>,
}

impl UserService {
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self { storage }
    }

    // 获取用户列表
    pub async fn list_users(&self, query: UserQueryParams) -> ActixResult<HttpResponse> {
        let list_query = UserListQuery {
            page: Some(query.pagination.page),
            size: Some(query.pagination.size),
            role: query.role,
            status: query.status,
            search: query.search,
        };

        match self.storage.list_users_with_pagination(list_query).await {
            Ok(response) => {
                Ok(HttpResponse::Ok().json(ApiResponse::success(response, "获取用户列表成功")))
            }
            Err(e) => Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    500,
                    format!("获取用户列表失败: {e}"),
                )),
            ),
        }
    }

    // 创建用户
    pub async fn create_user(&self, user_data: CreateUserRequest) -> ActixResult<HttpResponse> {
        match self.storage.create_user(user_data).await {
            Ok(user) => {
                Ok(HttpResponse::Created().json(ApiResponse::success(user, "用户创建成功")))
            }
            Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                400,
                format!("用户创建失败: {e}"),
            ))),
        }
    }

    // 根据ID获取用户
    pub async fn get_user(&self, user_id: i64) -> ActixResult<HttpResponse> {
        match self.storage.get_user_by_id(user_id).await {
            Ok(Some(user)) => {
                Ok(HttpResponse::Ok().json(ApiResponse::success(user, "获取用户信息成功")))
            }
            Ok(None) => {
                Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(404, "用户不存在")))
            }
            Err(e) => Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    500,
                    format!("获取用户信息失败: {e}"),
                )),
            ),
        }
    }

    // 更新用户信息
    pub async fn update_user(
        &self,
        user_id: i64,
        update_data: UpdateUserRequest,
    ) -> ActixResult<HttpResponse> {
        match self.storage.update_user(user_id, update_data).await {
            Ok(Some(user)) => {
                Ok(HttpResponse::Ok().json(ApiResponse::success(user, "用户信息更新成功")))
            }
            Ok(None) => {
                Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(404, "用户不存在")))
            }
            Err(e) => Ok(HttpResponse::BadRequest().json(ApiResponse::error_empty(
                400,
                format!("用户信息更新失败: {e}"),
            ))),
        }
    }

    // 删除用户
    pub async fn delete_user(&self, user_id: i64) -> ActixResult<HttpResponse> {
        match self.storage.delete_user(user_id).await {
            Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse::success_empty("用户删除成功"))),
            Ok(false) => {
                Ok(HttpResponse::NotFound().json(ApiResponse::error_empty(404, "用户不存在")))
            }
            Err(e) => Ok(
                HttpResponse::InternalServerError().json(ApiResponse::error_empty(
                    500,
                    format!("用户删除失败: {e}"),
                )),
            ),
        }
    }
}
