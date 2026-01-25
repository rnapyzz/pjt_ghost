use chrono::NaiveDate;
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::pl_entry::{PlEntry, PlEntryRepository, Scenario, UpsertPlEntryParam},
    error::AppError,
};

#[derive(Debug, Clone)]
pub struct PlEntryRepositoryImpl {
    pool: PgPool,
}

impl PlEntryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PlEntryRepository for PlEntryRepositoryImpl {
    async fn find_by_project(
        &self,
        project_id: Uuid,
        scenario: Scenario,
    ) -> Result<Vec<PlEntry>, AppError> {
        let entries = sqlx::query_as!(
            PlEntry,
            r#"
            SELECT
                id,
                project_id,
                job_id,
                scenario as "scenario: Scenario",
                date,
                account_item_id,
                amount,
                description,
                created_by,
                updated_by,
                created_at,
                updated_at
            FROM pl_entries
            WHERE project_id = $1 AND scenario = $2
            ORDER BY date ASC
            "#,
            project_id,
            scenario as Scenario
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch pl entries: {:?}", e);
            AppError::from(e)
        })?;

        Ok(entries)
    }

    async fn bulk_upsert(
        &self,
        project_id: Uuid,
        scenario: Scenario,
        entries: Vec<UpsertPlEntryParam>,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        if entries.is_empty() {
            return Ok(());
        }

        let account_item_ids: Vec<Uuid> = entries.iter().map(|e| e.account_item_id).collect();
        let dates: Vec<NaiveDate> = entries.iter().map(|e| e.date).collect();
        let amounts: Vec<Decimal> = entries.iter().map(|e| e.amount).collect();
        let descriptions: Vec<Option<String>> =
            entries.iter().map(|e| e.description.clone()).collect();

        sqlx::query!(
            r#"
            INSERT INTO pl_entries (
                project_id,
                scenario,
                account_item_id,
                date,
                amount,
                description,
                created_by,
                updated_by,
                created_at,
                updated_at
            )
            SELECT
                $1,
                $2,
                u.account_item_id,
                u.date,
                u.amount,
                u.description,
                $6, -- created_by
                $6, -- updated_by
                NOW(),
                NOW()
            FROM UNNEST(
                $3::uuid[],
                $4::date[],
                $5::numeric[],
                $7::text[]
            ) AS u(account_item_id, date, amount, description)
            ON CONFLICT (project_id, scenario, account_item_id, date) WHERE job_id IS NULL
            DO UPDATE SET
                amount = EXCLUDED.amount,
                description = EXCLUDED.description,
                updated_by = EXCLUDED.updated_by,
                updated_at = CURRENT_TIMESTAMP
            "#,
            project_id,
            scenario as Scenario,
            &account_item_ids,
            &dates,
            &amounts as &[Decimal],
            user_id,
            &descriptions as &[Option<String>]
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to bulk upsert to entries: {:?}", e);
            AppError::from(e)
        })?;

        Ok(())
    }
}
