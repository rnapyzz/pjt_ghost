use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::items::{Account, AccountCategory, Entry, Item, ItemRepository, ItemType};

pub struct ItemRepositoryImpl {
    pool: PgPool,
}

impl ItemRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemRepository for ItemRepositoryImpl {
    async fn list_accounts(&self) -> Result<Vec<Account>> {
        let accounts = sqlx::query_as!(
            Account,
            r#"
            SELECT
                id,
                name,
                category as "category: AccountCategory",
                created_at,
                updated_at
            FROM accounts
            ORDER BY category, id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }

    async fn list_item_types(&self, account_id: Option<Uuid>) -> Result<Vec<ItemType>> {
        let types = match account_id {
            Some(acc_id) => {
                sqlx::query_as!(
                    ItemType,
                    r#"
                    SELECT id, account_id, name, created_at, updated_at
                    FROM item_types
                    WHERE account_id = $1 ORDER BY id
                    "#,
                    acc_id
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    ItemType,
                    r#"
                    SELECT id, account_id, name, created_at, updated_at 
                    FROM item_types 
                    ORDER BY id
                    "#
                )
                .fetch_all(&self.pool)
                .await?
            }
        };
        Ok(types)
    }

    async fn create(
        &self,
        job_id: Uuid,
        item_type_id: Uuid,
        assignee_id: Option<Uuid>,
        name: String,
        description: String,
        entries: Vec<(NaiveDate, i64)>,
    ) -> Result<Item> {
        let mut tx = self.pool.begin().await?;

        let item_rec = sqlx::query!(
            r#"
            INSERT INTO items (job_id, item_type_id, assignee_id, name, description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, created_at, updated_at
            "#,
            job_id,
            item_type_id,
            assignee_id,
            name,
            description
        )
        .fetch_one(&mut *tx)
        .await?;

        let mut saved_entries = Vec::new();
        for (date, amount) in entries {
            let entry = sqlx::query_as!(
                Entry,
                r#"
            INSERT INTO entries (item_id, date, amount)
            VALUES ($1, $2, $3)
            RETURNING id, item_id, date, amount, created_at, updated_at
            "#,
                item_rec.id,
                date,
                amount
            )
            .fetch_one(&mut *tx)
            .await?;

            saved_entries.push(entry);
        }

        tx.commit().await?;

        Ok(Item {
            id: item_rec.id,
            job_id,
            item_type_id,
            assignee_id,
            name,
            description,
            entries: saved_entries,
            created_at: item_rec.created_at,
            updated_at: item_rec.updated_at,
        })
    }

    async fn find_by_job_id(&self, job_id: Uuid) -> Result<Vec<Item>> {
        let item_recs = sqlx::query!(
            r#"
            SELECT id, job_id, item_type_id, assignee_id, name, description, created_at, updated_at
            FROM items WHERE job_id = $1 ORDER BY created_at
            "#,
            job_id
        )
        .fetch_all(&self.pool)
        .await?;

        if item_recs.is_empty() {
            return Ok(vec![]);
        }

        let item_ids: Vec<Uuid> = item_recs.iter().map(|rec| rec.id).collect();
        let all_entries = sqlx::query_as!(
            Entry,
            r#"
            SELECT id, item_id, date, amount, created_at, updated_at
            FROM entries
            WHERE item_id = ANY($1)
            ORDER BY date
            "#,
            &item_ids
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for rec in item_recs {
            let my_entries: Vec<Entry> = all_entries
                .iter()
                .filter(|e| e.item_id == rec.id)
                .cloned()
                .collect();

            result.push(Item {
                id: rec.id,
                job_id: rec.job_id,
                item_type_id: rec.item_type_id,
                assignee_id: rec.assignee_id,
                name: rec.name,
                description: rec.description,
                entries: my_entries,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
            });
        }

        Ok(result)
    }

    async fn update(
        &self,
        item_id: Uuid,
        job_id: Uuid,
        assignee_id: Option<Uuid>,
        name: Option<String>,
        description: Option<String>,
        entries: Option<Vec<(NaiveDate, i64)>>,
    ) -> Result<Item> {
        // 現状の update は Delete-Insert 方式

        let mut tx = self.pool.begin().await?;

        // itemの更新
        sqlx::query!(
            r#"
            UPDATE items
            SET
                assignee_id = COALESCE($1, assignee_id),
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                updated_at = now()
            WHERE id = $4 AND job_id = $5 
            "#,
            assignee_id,
            name,
            description,
            item_id,
            job_id
        )
        .execute(&mut *tx)
        .await?;

        // entryの更新
        if let Some(new_entries) = entries {
            sqlx::query!("DELETE FROM entries WHERE item_id = $1", item_id)
                .execute(&mut *tx)
                .await?;

            for (date, amount) in new_entries {
                sqlx::query!(
                    "INSERT INTO entries (item_id, date, amount) VALUES ($1, $2, $3)",
                    item_id,
                    date,
                    amount
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        let item_rec = sqlx::query!(r#"SELECT * FROM items WHERE id = $1"#, item_id)
            .fetch_one(&self.pool)
            .await?;

        let entries_recs = sqlx::query_as!(
            Entry,
            r#"SELECT * FROM entries WHERE item_id = $1 ORDER BY date"#,
            item_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Item {
            id: item_rec.id,
            job_id: item_rec.job_id,
            item_type_id: item_rec.item_type_id,
            assignee_id: item_rec.assignee_id,
            name: item_rec.name,
            description: item_rec.description,
            entries: entries_recs,
            created_at: item_rec.created_at,
            updated_at: item_rec.updated_at,
        })
    }

    async fn delete(&self, item_id: Uuid, job_id: Uuid) -> Result<u64> {
        let mut tx = self.pool.begin().await?;

        // 紐づいているEntryを削除
        sqlx::query!("DELETE FROM entries WHERE item_id = $1", item_id)
            .execute(&mut *tx)
            .await?;

        // Itemを削除
        let result = sqlx::query!(
            "DELETE FROM items WHERE id = $1 AND job_id = $2",
            item_id,
            job_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(result.rows_affected())
    }
}
