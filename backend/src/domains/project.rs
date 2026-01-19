use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectType {
    Normal,
    Agile,
    Maintenance,
    RandD,
    Operation,
    Stock,
}

impl fmt::Display for ProjectType {
    // Stringへの変換をするためにfmtメソッドを実装しておく
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for ProjectType {
    fn default() -> Self {
        ProjectType::Normal
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub theme_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,

    pub project_type: String,
    pub attributes: Json<serde_json::Value>,

    pub is_active: bool,
    pub owner_id: Option<Uuid>,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn type_enum(&self) -> ProjectType {
        serde_json::from_value(serde_json::Value::String(self.project_type.clone()))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub struct CreateProjectParam {
    pub theme_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,

    pub project_type: ProjectType,
    pub attributes: Option<serde_json::Value>,

    pub owner_id: Option<Uuid>,
    pub created_by: Uuid,
}

#[async_trait::async_trait]
pub trait ProjectRepositoryImpl: Send + Sync {
    async fn create(&self, params: CreateProjectParam) -> Result<Project, AppError>;
    async fn find_all(&self) -> Result<Vec<Project>, AppError>;
}
