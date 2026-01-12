use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub employee_id: String,
    pub username: String,
    pub name: String,
    pub email: String,

    #[serde(skip)]
    pub password_hash: String,

    pub role: String,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 権限
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    General,
    Admin,
    Manager,
}

impl UserRole {
    /// UserRole構造体から文字列スライスに変換するメソッド
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::General => "general",
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
        }
    }
}

/// UserRoleのデフォルト値を設定する関数
impl Default for UserRole {
    fn default() -> Self {
        Self::General
    }
}

/// ユーザー作成時のパラメーター
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserParam {
    pub employee_id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<UserRole>,
}

impl CreateUserParam {
    pub fn hash_password(&self) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(self.password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashin failed: {}", e))?
            .to_string())
    }
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, params: CreateUserParam) -> Result<User, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
}
