use crate::models::homeworks::responses::HomeworkListResponse;
use crate::models::homeworks::requests::HomeworkListQuery;
use crate::errors::Result;
use sqlx::PgPool;

pub struct HomeworkRepository {
    pool: PgPool,
}

impl HomeworkRepository {
    pub fn new(pool: PgPool) -> Self {
        HomeworkRepository { pool }
    }

    pub async fn get_homework_list(
        &self,
        query: HomeworkListQuery,
    ) -> Result<HomeworkListResponse> {
        let mut db_query = sqlx::query_as!(
            crate::models::homeworks::entities::Homework,
            r#"
            SELECT * FROM homeworks
            WHERE
                ($1::text IS NULL OR title ILIKE $1)
                AND ($2::text IS NULL OR description ILIKE $2)
                AND ($3::int4 IS NULL OR class_id = $3)
                AND ($4::int4 IS NULL OR teacher_id = $4)
            ORDER BY created_at DESC
            LIMIT $5
            OFFSET $6
            "#,
            query.title.as_deref(),
            query.description.as_deref(),
            query.class_id,
            query.teacher_id,
            query.limit as i64,
            query.offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let total = db_query.len() as i64;

        Ok(HomeworkListResponse {
            total,
            items: db_query,
        })
    }
}