use sqlx::{PgPool, types::Json};

use crate::{
    domains::project::{CreateProjectParam, Project, ProjectRepository},
    error::AppError,
};

#[derive(Debug, Clone)]
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
    async fn create(&self, params: CreateProjectParam) -> Result<Project, AppError> {
        let type_str = params.project_type.to_string();

        let attributes = Json(params.attributes.unwrap_or(serde_json::json!({})));

        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (
                theme_id,
                name,
                description,
                type,
                attributes,
                owner_id,
                created_by,
                updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $7)
            RETURNING
                id,
                theme_id,
                name,
                description,
                type as "project_type",
                attributes as "attributes: Json<serde_json::Value>",
                is_active,
                owner_id,
                created_by,
                updated_by,
                created_at,
                updated_at
            "#,
            params.theme_id,
            params.name,
            params.description,
            type_str,
            attributes as Json<serde_json::Value>,
            params.owner_id,
            params.created_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            AppError::from(e)
        })?;

        Ok(project)
    }

    async fn find_all(&self) -> Result<Vec<Project>, AppError> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT
                id, 
                theme_id,
                name,
                description,
                type as "project_type",
                attributes as "attributes: Json<serde_json::Value>",
                is_active,
                owner_id,
                created_by,
                updated_by,
                created_at,
                updated_at
            FROM projects
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(projects)
    }
}
