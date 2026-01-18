use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SegmentUiConfig {
    pub icon: Option<String>,
    pub color_theme: Option<String>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Segment {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub ui_config: Json<SegmentUiConfig>,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSegmentParam {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub ui_config: Option<SegmentUiConfig>,
    pub created_by: Uuid,
}

#[async_trait::async_trait]
pub trait SegmentRepository: Send + Sync {
    async fn create(&self, params: CreateSegmentParam) -> Result<Segment, AppError>;
    async fn find_all(&self) -> Result<Vec<Segment>, AppError>;
}
