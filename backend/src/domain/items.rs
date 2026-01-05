use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Type, prelude::FromRow};
use uuid::Uuid;

// ------------------------------------------
// 勘定科目のカテゴリ（売上、売上原価、販管費）
// ------------------------------------------
#[derive(Debug, Deserialize, Serialize, Type, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "account_category", rename_all = "snake_case")]
pub enum AccountCategory {
    Sales,
    CostOfSales,
    Sga,
}

// ------------------------------------------
// 勘定科目（マスタ）
// ------------------------------------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub category: AccountCategory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ------------------------------------------
// 項目種別（マスタ）
// ------------------------------------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ItemType {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ------------------------------------------
// 計上データ
// ------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Entry {
    pub id: Uuid,
    pub item_id: Uuid,
    pub date: NaiveDate,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ------------------------------------------
// 項目エンティティ
// ------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub job_id: Uuid,
    pub item_type_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub entries: Vec<Entry>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait ItemRepository: Send + Sync {
    // マスタ取得系メソッド
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn list_item_types(&self, account_id: Option<Uuid>) -> Result<Vec<ItemType>>;

    // データ操作系メソッド
    async fn create(
        &self,
        job_id: Uuid,
        item_type_id: Uuid,
        assignee_id: Option<Uuid>,
        name: String,
        description: String,
        entries: Vec<(NaiveDate, i64)>,
    ) -> Result<Item>;
    async fn find_by_job_id(&self, job_id: Uuid) -> Result<Vec<Item>>;
    async fn update(
        &self,
        item_id: Uuid,
        job_id: Uuid,
        // item_type_id: Option<Uuid>,  // 項目タイプは変更不可
        assignee_id: Option<Uuid>,
        name: Option<String>,
        description: Option<String>,
        entries: Option<Vec<(NaiveDate, i64)>>,
    ) -> Result<Item>;
    async fn delete(&self, item_id: Uuid, job_id: Uuid) -> Result<u64>;
}
