use sqlx::{PgPool, types::Json};

use crate::{
    domains::segment::{CreateSegmentParam, Segment, SegmentRepository, SegmentUiConfig},
    error::AppError,
};

#[derive(Debug, Clone)]
pub struct SegmentRepositoryImpl {
    pool: PgPool,
}

impl SegmentRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SegmentRepository for SegmentRepositoryImpl {
    async fn create(&self, params: CreateSegmentParam) -> Result<Segment, AppError> {
        let ui_config = Json(params.ui_config.unwrap_or_default());

        let segment = sqlx::query_as!(
            Segment,
            r#"
            INSERT INTO segments (
               slug,
               name,
               description,
               ui_config,
               created_by,
               updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $5)
            RETURNING
                id,
                slug,
                name,
                description,
                ui_config as "ui_config!: Json<SegmentUiConfig>",
                created_by,
                updated_by,
                created_at,
                updated_at
            "#,
            params.slug,
            params.name,
            params.description,
            ui_config as Json<SegmentUiConfig>,
            params.created_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Fatabase error: {:?}", e);
            AppError::from(e)
        })?;

        Ok(segment)
    }

    async fn find_all(&self) -> Result<Vec<Segment>, AppError> {
        let segments = sqlx::query_as!(
            Segment,
            r#"
            SELECT
                id,
                slug,
                name,
                description,
                ui_config as "ui_config!: Json<SegmentUiConfig>",
                created_by,
                updated_by,
                created_at,
                updated_at
            FROM segments
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(segments)
    }
}
