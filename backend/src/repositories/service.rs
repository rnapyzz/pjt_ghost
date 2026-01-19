use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::service::{CreateServiceParam, Service, ServiceRepository, UpdateServiceParam},
    error::AppError,
};

#[derive(Debug, Clone)]
pub struct ServiceRepositoryImpl {
    pool: PgPool,
}

impl ServiceRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ServiceRepository for ServiceRepositoryImpl {
    async fn create(&self, params: CreateServiceParam) -> Result<Service, AppError> {
        let service = sqlx::query_as!(
            Service,
            r#"
            INSERT INTO services (
                slug,
                name,
                owner_id,
                segment_id,
                created_by,
                updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $5)
            RETURNING *
            "#,
            params.slug,
            params.name,
            params.owner_id,
            params.segment_id,
            params.created_by,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {:?}", e);
            AppError::from(e)
        })?;

        Ok(service)
    }

    async fn find_all(&self) -> Result<Vec<Service>, AppError> {
        let services = sqlx::query_as!(
            Service,
            r#"
            SELECT * FROM services ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(services)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Service>, AppError> {
        let service = sqlx::query_as!(
            Service,
            r#"
            SELECT * FROM services
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(service)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Service>, AppError> {
        let service = sqlx::query_as!(
            Service,
            r#"
            SELECT * FROM services
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(service)
    }

    async fn update(&self, id: Uuid, params: UpdateServiceParam) -> Result<Service, AppError> {
        let service = sqlx::query_as!(
            Service,
            r#"
            UPDATE services
            SET
                slug = COALESCE($1, slug),
                name = COALESCE($2, name),
                owner_id = COALESCE($3, owner_id),
                segment_id = COALESCE($4, segment_id),
                updated_by = $5,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $6
            RETURNING *
            "#,
            params.slug,
            params.name,
            params.owner_id,
            params.segment_id,
            params.updated_by,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update service: {:?}", e);
            AppError::from(e)
        })?
        .ok_or(AppError::NotFound(format!("Service {} not found", id)))?;

        Ok(service)
    }
    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(r#"DELETE FROM services WHERE id = $1"#, id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Service {} not found", id)));
        }

        Ok(())
    }
}
