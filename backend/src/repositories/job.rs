use sqlx::PgPool;

use crate::{
    domains::job::{CreateJobParam, Job, JobRepository},
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
}
