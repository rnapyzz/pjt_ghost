use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Type, prelude::FromRow};
use uuid::Uuid;

// ------------------------------------------
// ビジネスモデル（DBのEnumとのマッピング、UIに表示する単位）
// ------------------------------------------
#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "business_model_type", rename_all = "snake_case")]
pub enum BusinessModel {
    Contract,
    Ses,
    Saas,
    Media,
    Internal,
    Rnd,
}

// ------------------------------------------
// 収益構造タイプ(システム内部の抽象化用の型)
// ------------------------------------------
pub enum RevenueType {
    Flow,
    Stock,
    Internal,
}

// ロジックのためにビジネスモデルを抽象化するメソッド
impl BusinessModel {
    pub fn revenue_type(&self) -> RevenueType {
        match self {
            BusinessModel::Contract | BusinessModel::Ses => RevenueType::Flow,
            BusinessModel::Saas | BusinessModel::Media => RevenueType::Stock,
            BusinessModel::Internal | BusinessModel::Rnd => RevenueType::Internal,
        }
    }
}

// ------------------------------------------
// Jobエンティティ
// ------------------------------------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Job {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
    pub business_model: BusinessModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn create(
        &self,
        project_id: Uuid,
        name: String,
        description: String,
        business_model: BusinessModel,
    ) -> Result<Job>;
    async fn find_by_project_id(&self, project_id: Uuid) -> Result<Vec<Job>>;
    async fn update(
        &self,
        id: Uuid,
        project_id: Uuid,
        name: Option<String>,
        description: Option<String>,
        business_model: Option<BusinessModel>,
    ) -> Result<Job>;
    async fn delete(&self, id: Uuid, project_id: Uuid) -> Result<u64>;
}
