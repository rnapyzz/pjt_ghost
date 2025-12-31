use anyhow::Result;
use sqlx::PgPool;

use crate::domain::users::{User, UserRepository};

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, name: String) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name)
            VALUES ($1)
            RETURNING id, name
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
