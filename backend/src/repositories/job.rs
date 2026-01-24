use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::job::{CreateJobParam, Job, JobRepository, UpdateJobParam},
    error::AppError,
};

#[derive(Debug, Clone)]
pub struct JobRepositoryImpl {
    pool: PgPool,
}

impl JobRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl JobRepository for JobRepositoryImpl {
    async fn create(&self, params: CreateJobParam) -> Result<Job, AppError> {
        let theme_id = if params.project_id.is_some() {
            None
        } else {
            params.theme_id
        };

        let job = sqlx::query_as!(
            Job,
            r#"
            INSERT INTO jobs
            (
                service_id,
                project_id,
                theme_id,
                title,
                description,
                status,
                owner_id,
                created_by,
                updated_by
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            RETURNING *
            "#,
            params.service_id,
            params.project_id,
            theme_id,
            params.title,
            params.description,
            params.status,
            params.owner_id,
            params.created_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            AppError::from(e)
        })?;

        Ok(job)
    }

    async fn find_all(&self) -> Result<Vec<Job>, AppError> {
        let jobs = sqlx::query_as!(
            Job,
            r#"
            SELECT * FROM jobs ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Job>, AppError> {
        let job = sqlx::query_as!(
            Job,
            r#"
            SELECT * FROM jobs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(job)
    }

    async fn update(&self, id: Uuid, params: UpdateJobParam) -> Result<Job, AppError> {
        let job = sqlx::query_as!(
            Job,
            r#"
        UPDATE jobs
        SET
            service_id = COALESCE($1, service_id),
            project_id = COALESCE($2, project_id),
            theme_id = COALESCE($3, theme_id),
            title = COALESCE($4, title),
            description = COALESCE($5, description),
            status = COALESCE($6, status),
            owner_id = COALESCE($7, owner_id),
            updated_by = $8,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = $9
        RETURNING *
        "#,
            params.service_id,
            params.project_id,
            params.theme_id,
            params.title,
            params.description,
            params.status,
            params.owner_id,
            params.updated_by,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM jobs
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Job {} not found", id)));
        }

        Ok(())
    }
}
