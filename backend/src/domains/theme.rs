use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Theme {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub segment_id: Option<Uuid>,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateThemeParam {
    pub title: String,
    pub description: Option<String>,
    pub segment_id: Option<Uuid>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateThemeParam {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub segment_id: Option<Uuid>,
    pub updated_by: Uuid,
}

#[async_trait::async_trait]
pub trait ThemeRepository: Send + Sync {
    async fn create(&self, params: CreateThemeParam) -> Result<Theme, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Theme>, AppError>;
    async fn find_all(&self) -> Result<Vec<Theme>, AppError>;
    async fn update(&self, id: Uuid, params: UpdateThemeParam) -> Result<Theme, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
