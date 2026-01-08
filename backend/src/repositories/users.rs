use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

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
    async fn create(&self, name: String, email: String, password_hash: String) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password_hash, role)
            VALUES ($1, $2, $3, 'member')
            RETURNING *;
            "#,
            name,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE email = $1"#, email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
