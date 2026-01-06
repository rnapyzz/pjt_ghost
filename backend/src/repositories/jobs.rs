use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::jobs::{BusinessModel, Job, JobRepository};

pub struct JobRepositoryImpl {
    pool: PgPool,
}

impl JobRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobRepository for JobRepositoryImpl {
    async fn create(
        &self,
        project_id: Uuid,
        name: String,
        description: String,
        business_model: BusinessModel,
    ) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
            INSERT INTO jobs (project_id, name, description, business_model)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                project_id,
                name,
                description,
                business_model as "business_model: BusinessModel",
                created_at,
                updated_at
            "#,
            project_id,
            name,
            description,
            business_model as BusinessModel
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    async fn find_by_project_id(&self, project_id: Uuid) -> Result<Vec<Job>> {
        let jobs = sqlx::query_as!(
            Job,
            r#"
            SELECT
                id,
                project_id,
                name,
                description,
                business_model as "business_model: BusinessModel",
                created_at,
                updated_at
            FROM jobs
            WHERE project_id = $1
            ORDER BY created_at DESC
            "#,
            project_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(jobs)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Job>> {
        let job = sqlx::query_as!(
            Job,
            r#"
            SELECT
                id,
                project_id,
                name,
                description,
                business_model as "business_model: BusinessModel",
                created_at,
                updated_at
            FROM jobs
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(job)
    }

    async fn update(
        &self,
        id: Uuid,
        project_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        business_model: Option<BusinessModel>,
    ) -> Result<Job> {
        let job = sqlx::query_as!(
            Job,
            r#"
            UPDATE jobs
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                business_model = COALESCE($3, business_model),
                updated_at = NOW()
            WHERE id = $4 AND project_id = $5
            RETURNING
                id,
                project_id,
                name,
                description,
                business_model as "business_model: BusinessModel",
                created_at,
                updated_at
            "#,
            name,
            description,
            business_model as Option<BusinessModel>,
            id,
            project_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(job)
    }

    async fn delete(&self, id: Uuid, project_id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM jobs
            WHERE id = $1 AND project_id = $2
            "#,
            id,
            project_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
