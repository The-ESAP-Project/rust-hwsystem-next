use sqlx::{FromRow, Row};

use super::SqliteStorage;
use crate::errors::{HWSystemError, Result};
use crate::models::class_student::entities::{ClassStudent, ClassUserRole};
use crate::models::classes::entities::Class;

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

pub async fn get_class_and_user_student_by_id_and_code(
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
