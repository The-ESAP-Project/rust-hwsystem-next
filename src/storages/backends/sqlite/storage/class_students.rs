use sqlx::{FromRow, Row};

use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::PaginationInfo;
use crate::models::class_student::entities::{ClassStudent, ClassUserRole};
use crate::models::classes::{
    entities::Class, requests::ClassListQuery, responses::ClassListResponse,
};

pub async fn join_class(
    storage: &SqliteStorage,
    user_id: i64,
    class_id: i64,
) -> Result<ClassStudent> {
    let now = chrono::Utc::now().timestamp();

    // 插入关联
    let class_student = sqlx::query_as::<sqlx::Sqlite, ClassStudent>(
        "INSERT INTO class_students (class_id, student_id, role, joined_at)
        VALUES (?, ?, ?, ?) RETURNING *",
    )
    .bind(class_id)
    .bind(user_id)
    .bind(ClassUserRole::Student.to_string())
    .bind(now)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to join class: {e}")))?;

    Ok(class_student)
}

pub async fn list_user_classes_with_pagination(
    storage: &SqliteStorage,
    user_id: i64,
    query: ClassListQuery,
) -> Result<ClassListResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let size = query.size.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * size;

    let total = sqlx::query_scalar::<sqlx::Sqlite, i64>(
        "SELECT COUNT(*) FROM class_students WHERE student_id = ?",
    )
    .bind(user_id)
    .fetch_one(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to count user classes: {e}")))?;

    let classes = sqlx::query_as::<sqlx::Sqlite, Class>(
        "SELECT c.* FROM classes c
        JOIN class_students cs ON cs.class_id = c.id
        WHERE cs.student_id = ?
        LIMIT ? OFFSET ?",
    )
    .bind(user_id)
    .bind(size)
    .bind(offset)
    .fetch_all(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to list user classes: {e}")))?;

    Ok(ClassListResponse {
        items: classes,
        pagination: PaginationInfo {
            page,
            size,
            total,
            pages: (total + size - 1) / size, // 向上取整
        },
    })
}

pub async fn get_user_class_role(
    storage: &SqliteStorage,
    user_id: i64,
    class_id: i64,
) -> Result<Option<ClassStudent>> {
    let class_student = sqlx::query_as::<sqlx::Sqlite, ClassStudent>(
        "SELECT role FROM class_students WHERE student_id = ? AND class_id = ?",
    )
    .bind(user_id)
    .bind(class_id)
    .fetch_optional(&storage.pool)
    .await
    .map_err(|e| HWSystemError::database_operation(format!("Failed to get class_student: {e}")))?;

    Ok(class_student)
}

pub async fn get_class_and_class_student_by_id_and_code(
    storage: &SqliteStorage,
    class_id: i64,
    invite_code: &str,
    user_id: i64,
) -> Result<(Option<Class>, Option<ClassStudent>)> {
    let row = sqlx::query(
        "SELECT c.*,
        cs.id as cs_id,
        cs.class_id as cs_class_id,
        cs.student_id as cs_student_id,
        cs.role as cs_role,
        cs.joined_at as cs_joined_at
        FROM classes c
        LEFT JOIN class_students cs ON cs.class_id = c.id AND cs.student_id = ?
        WHERE c.id = ? AND c.invite_code = ?",
    )
    .bind(user_id)
    .bind(class_id)
    .bind(invite_code)
    .fetch_optional(&storage.pool)
    .await
    .map_err(|e| {
        HWSystemError::database_operation(format!(
            "Failed to get class and user student by id and code: {e}"
        ))
    })?;

    if let Some(row) = row {
        let class = Class::from_row(&row).map_err(|e| {
            HWSystemError::database_operation(format!("Failed to decode class: {e}"))
        })?;

        // 只要 cs_id 不为 null 就说明有成员信息
        let class_student = row
            .try_get::<i64, _>("cs_id")
            .ok()
            .and_then(|_| ClassStudent::from_row_prefix("cs_", &row).ok());

        tracing::debug!(class = ?class, class_student = ?class_student);
        Ok((Some(class), class_student))
    } else {
        Ok((None, None))
    }
}
