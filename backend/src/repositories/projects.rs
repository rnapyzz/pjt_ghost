use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::projects::{Project, ProjectRepository};

#[derive(Clone)]
pub struct ProjectRepositoryImpl {
    pool: PgPool,
}

impl ProjectRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ProjectRepository for ProjectRepositoryImpl {
    async fn create(&self, name: String, description: String, owner_id: Uuid) -> Result<Project> {
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (name, description, owner_id)
            VALUES ($1, $2, $3)
            RETURNING id, name, description, owner_id, created_at, updated_at
            "#,
            name,
            description,
            owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    async fn list(&self, owner_id: Uuid) -> Result<Vec<Project>> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT id, name, description, owner_id, created_at, updated_at
            FROM projects
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(projects)
    }

    async fn update(
        &self,
        id: Uuid,
        name: String,
        description: String,
        owner_id: Uuid,
    ) -> Result<Project> {
        let project = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET name = $1, description = $2, updated_at = CURRENT_TIMESTAMP
            WHERE id = $3 AND owner_id = $4
            RETURNING id, name, description, owner_id, created_at, updated_at
            "#,
            name,
            description,
            id,
            owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    async fn delete(&self, id: Uuid, owner_id: Uuid) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1 AND owner_id = $2
            "#,
            id,
            owner_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
