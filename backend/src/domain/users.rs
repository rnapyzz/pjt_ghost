use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, name: String) -> Result<User>;
}
