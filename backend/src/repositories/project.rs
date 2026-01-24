use sqlx::{PgPool, types::Json};
use uuid::Uuid;

use crate::{
    domains::project::{CreateProjectParam, Project, ProjectRepository, UpdateProjectParam},
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
                attributes,
                type,
                target_market,
                value_prop,
                target_client,
                kpis,
                owner_id,
                created_by,
                updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            RETURNING
                id,
                theme_id,
                name,
                description,
                attributes as "attributes: Json<serde_json::Value>",
                type as "project_type",
                target_market,
                value_prop,
                target_client,
                kpis,
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
            attributes as Json<serde_json::Value>,
            type_str,
            params.target_market,
            params.value_prop,
            params.target_client,
            params.kpis,
            params.owner_id,
            params.created_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create project: {:?}", e);
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
                attributes as "attributes: Json<serde_json::Value>",
                type as "project_type",
                target_market,
                value_prop,
                target_client,
                kpis,
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
        .await
        .map_err(|e| {
            tracing::error!("Failed to find projects: {:?}", e);
            AppError::from(e)
        })?;

        Ok(projects)
    }

    async fn update(&self, id: Uuid, params: UpdateProjectParam) -> Result<Project, AppError> {
        let type_str = params.project_type.map(|t| t.to_string());

        let project = sqlx::query_as!(
            Project,
            r#"
            UPDATE projects
            SET
                theme_id = COALESCE($1, theme_id),
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                attributes = COALESCE($4, attributes),
                type = COALESCE($5, type),
                target_market = COALESCE($6, target_market),
                value_prop = COALESCE($7, value_prop),
                target_client = COALESCE($8, target_client),
                kpis = COALESCE($9, kpis),
                is_active = COALESCE($10, is_active),
                owner_id = COALESCE($11, owner_id),
                updated_by = $12,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $13
            RETURNING
                id, 
                theme_id,
                name,
                description,
                attributes as "attributes: Json<serde_json::Value>",
                type as "project_type",
                target_market,
                value_prop,
                target_client,
                kpis,
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
            params.attributes,
            type_str,
            params.target_market,
            params.value_prop,
            params.target_client,
            params.kpis,
            params.is_active,
            params.owner_id,
            params.updated_by,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Project {} not found", id)));
        }

        Ok(())
    }
}
