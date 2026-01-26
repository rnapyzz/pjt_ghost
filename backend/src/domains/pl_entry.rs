use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::{Type, prelude::FromRow};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, PartialEq, Type)]
#[sqlx(type_name = "scenario_type", rename_all = "PascalCase")]
pub enum Scenario {
    MasterPlan,     // 期初計画（設定後は変えられない）
    RevisedPlan,    // 改訂計画（設定後は変えられない）
    InitialPlan,    // 期初計画や改訂計画後に作られたProjectの初期計画（設定後は変えられない）
    ExecPlanAdjust, // ExecPlanをProject単位でマージするときの調整額
    JobPlan,        // Job立ち上げ時の初期計画（Project予算の消化）
    Actual,         // Jobの実績
}

#[derive(Debug, Clone, FromRow)]
pub struct PlEntry {
    pub id: Uuid,

    pub project_id: Uuid,
    pub job_id: Option<Uuid>,

    pub scenario: Scenario,
    pub date: NaiveDate,
    pub account_item_id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,

    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertPlEntryParam {
    pub account_item_id: Uuid,
    pub date: NaiveDate,
    pub amount: Decimal,
    pub description: Option<String>,
}

#[async_trait::async_trait]
pub trait PlEntryRepository: Send + Sync {
    async fn find_by_project(
        &self,
        project_id: Uuid,
        scenario: Scenario,
    ) -> Result<Vec<PlEntry>, AppError>;

    async fn bulk_upsert(
        &self,
        project_id: Uuid,
        scenario: Scenario,
        entries: Vec<UpsertPlEntryParam>,
        user_id: Uuid,
    ) -> Result<(), AppError>;
}
