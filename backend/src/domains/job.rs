use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Job {
    pub id: Uuid,
    pub service_id: Uuid,
    pub project_id: Option<Uuid>,
    pub theme_id: Option<Uuid>,

    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub owner_id: Option<Uuid>,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateJobParam {
    pub service_id: Uuid,
    pub project_id: Option<Uuid>,
    pub theme_id: Option<Uuid>,

    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub owner_id: Option<Uuid>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone)]
pub struct UpdateJobParam {
    pub service_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub theme_id: Option<Uuid>,

    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub owner_id: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[async_trait::async_trait]
pub trait JobRepository: Send + Sync {
    async fn create(&self, params: CreateJobParam) -> Result<Job, AppError>;
    async fn find_all(&self) -> Result<Vec<Job>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Job>, AppError>;
    async fn update(&self, id: Uuid, params: UpdateJobParam) -> Result<Job, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
