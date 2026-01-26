use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "account_category", rename_all = "snake_case")]
pub enum AccountType {
    Revenue,
    CostOfGoodsSold,
    SellingGeneralAdmin,
    NonOperationg,
    Tax,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountItem {
    pub id: Uuid,
    pub name: String,
    #[sqlx(rename = "account_type")]
    pub account_type: AccountType,
    pub display_order: i32,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait AccountItemRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<AccountItem>, AppError>;
}
