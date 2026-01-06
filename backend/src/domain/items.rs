//! Item（項目） に関するデータとドメインロジックの定義。
//!
//! このモジュールは、採算管理の主体となる [Job] を構成する要素である `Item` 構造体と、
//! それを操作するためのリポジトリインターフェースを提供します。

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Type, prelude::FromRow};
use uuid::Uuid;

/// 「勘定科目」の分類を表します。
///
/// `AccountCategory` は本バージョンでは3つを持っています。
/// - `Sales`: 売上高
/// - `CostOfSales`: 売上原価
/// - `Sga`: 販管費 (Selling, General and Administrative Expenses)
#[derive(Debug, Deserialize, Serialize, Type, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "account_category", rename_all = "snake_case")]
pub enum AccountCategory {
    Sales,
    CostOfSales,
    Sga,
}

/// 「勘定科目」を表します。
///
/// 各勘定科目`Account`はそれぞれいずれか1つの [AccountCategory] に分類され、
/// 各勘定科目には複数の [ItemType] が紐づきます。
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    pub category: AccountCategory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 「項目」の種類を表します。
///
/// 一般的に「費目」と称される単位ですが、売上要素の場合もあるので「項目」と表現します。
/// 勘定科目 [Account] はより会計的な分類にですが、実際の活動場面においてはやや抽象的であるため
/// 勘定科目を具体化した要素として `ItemType` 設定し、ユーザーにとって実用的な構成にすることをサポートします。
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ItemType {
    pub id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 月次単位の「計上データ」を表します。
///
/// 計上月と計上金額のセットデータです。
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Entry {
    pub id: Uuid,
    pub item_id: Uuid,
    pub date: NaiveDate,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 採算管理の最小単位となる「項目」を表します。
///
/// 1つの `Item` は、複数の月次計上データである [Entry] を保持します。
/// [ItemRepository::update] を使用することで、子要素 [Entry] の洗い替え更新が可能です。
/// （子要素 [Entry] の更新は洗い替え方式のみ）
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
    async fn list_accounts(&self) -> Result<Vec<Account>>;
    async fn list_item_types(&self, account_id: Option<Uuid>) -> Result<Vec<ItemType>>;

    /// 新しい [Item] を作成します。
    ///
    /// # Arguments
    ///
    /// * `job_id` - 紐づける [Job] のID
    /// * `entries` - 初回作成時の月次計上データリスト（空も可）
    ///
    /// # Errors
    ///
    /// データベースへの接続に失敗した場合や、外部キー制約違反（存在しない `jonb_id` 指定など）
    /// の場合に [sqlx::Error] をラップしたエラーを返します。
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
    async fn update_entries(&self, id: Uuid, entries: Vec<(NaiveDate, i64)>) -> Result<()>;
    async fn delete(&self, item_id: Uuid, job_id: Uuid) -> Result<u64>;
}
