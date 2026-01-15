use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::theme::{CreateThemeParam, Theme, ThemeRepository, UpdateThemeParam},
    error::AppError,
};

#[derive(Debug, Clone)]
pub struct ThemeRepositoryImpl {
    pool: PgPool,
}

impl ThemeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ThemeRepository for ThemeRepositoryImpl {
    async fn create(&self, params: CreateThemeParam) -> Result<Theme, AppError> {
        let theme = sqlx::query_as!(
            Theme,
            r#"
            INSERT INTO themes (title, description, created_by, updated_by)
            VALUES ($1, $2, $3, $3)
            RETURNING *
            "#,
            params.title,
            params.description,
            params.created_by,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(theme)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Theme>, AppError> {
        let theme = sqlx::query_as!(
            Theme,
            r#"
            SELECT * FROM themes WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(theme)
    }

    async fn find_all(&self) -> Result<Vec<Theme>, AppError> {
        let themes = sqlx::query_as!(
            Theme,
            r#"
            SELECT * FROM themes ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(themes)
    }

    async fn update(&self, id: Uuid, params: UpdateThemeParam) -> Result<Theme, AppError> {
        let theme = sqlx::query_as!(
            Theme,
            r#"
            UPDATE themes
            SET
                title = COALESCE($1, title),
                description = COALESCE($2, description),
                is_active = COALESCE($3, is_active),
                updated_by = $4,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $5
            RETURNING *
            "#,
            params.title,
            params.description,
            params.is_active,
            params.updated_by,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound(format!("Theme {} not found", id)))?;

        Ok(theme)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM themes
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Theme {} not found", id)));
        }

        Ok(())
    }
}
