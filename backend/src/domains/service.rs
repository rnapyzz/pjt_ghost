use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Service {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub owner_id: Option<Uuid>,
    pub segment_id: Uuid,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateServiceParam {
    pub slug: String,
    pub name: String,
    pub owner_id: Option<Uuid>,
    pub segment_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateServiceParam {
    pub slug: Option<String>,
    pub name: Option<String>,
    pub owner_id: Option<Uuid>,
    pub segment_id: Option<Uuid>,
    pub updated_by: Uuid,
}

#[async_trait::async_trait]
pub trait ServiceRepository: Send + Sync {
    async fn create(&self, params: CreateServiceParam) -> Result<Service, AppError>;
    async fn find_all(&self) -> Result<Vec<Service>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Service>, AppError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Service>, AppError>;
    async fn update(&self, id: Uuid, params: UpdateServiceParam) -> Result<Service, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
