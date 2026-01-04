use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::items::{Account, AccountCategory, ItemRepository, ItemType};

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
}
