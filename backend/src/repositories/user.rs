use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::user::{CreateUserParam, User, UserRepository},
    error::AppError,
};

/// UserRepositoryの実装構造体
#[derive(Debug, Clone)]
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
    async fn create(&self, params: CreateUserParam) -> Result<User, AppError> {
        let password_hash = params.hash_password()?;
        let role_str = params.role.unwrap_or_default().as_str();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                employee_id,
                username,
                name,
                email,
                password_hash,
                role
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
            params.employee_id,
            params.username,
            params.name,
            params.email,
            password_hash,
            role_str
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
