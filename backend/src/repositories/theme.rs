use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::theme::{CreateThemeParam, Theme, ThemeRepository},
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
}
