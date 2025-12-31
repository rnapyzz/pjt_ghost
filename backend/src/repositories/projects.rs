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
            RETURNING id, name, description, owner_id
            "#,
            name,
            description,
            owner_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }
}
