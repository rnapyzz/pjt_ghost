use crate::{
    domains::account_item::{AccountItem, AccountItemRepository, AccountType},
    error::AppError,
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AccountItemRepositoryImpl {
    pool: PgPool,
}

impl AccountItemRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AccountItemRepository for AccountItemRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<AccountItem>, AppError> {
        let items = sqlx::query_as!(
            AccountItem,
            r#"
            SELECT
                id,
                name,
                account_type as "account_type: AccountType",
                display_order,
                description,
                is_active,
                created_at,
                updated_at
            FROM account_items
            ORDER BY display_order ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch account items: {:?}", e);
            AppError::from(e)
        })?;

        Ok(items)
    }
}
