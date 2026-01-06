use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn create(&self, name: String, description: String, owner_id: Uuid) -> Result<Project>;
    async fn list(&self, owner_id: Uuid) -> Result<Vec<Project>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>>;
    async fn update(
        &self,
        id: Uuid,
        name: String,
        description: String,
        owner_id: Uuid,
    ) -> Result<Project>;
    async fn delete(&self, id: Uuid, owner_id: Uuid) -> Result<u64>;
}
