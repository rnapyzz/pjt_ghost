use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    domains::user::User,
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub iat: usize,
    pub exp: usize,
    pub name: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await?
        .ok_or(AppError::AuthError)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| anyhow::anyhow!("Invalid password hash in DB: {}", e))?;

    let is_valid = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err(AppError::AuthError);
    }

    let issue_at = Utc::now().timestamp();
    let expire_at = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role,
        iat: issue_at as usize,
        exp: expire_at as usize,
        name: user.name.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| anyhow::anyhow!("Token creation failed: {}", e))?;

    Ok(Json(LoginResponse { token }))
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub employee_id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            employee_id: user.employee_id,
            username: user.username,
            name: user.name,
            email: user.email,
            role: user.role,
        }
    }
}

pub async fn get_current_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<UserResponse>> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let user = state
        .user_repository
        .find_by_id(user_id)
        .await
        .map_err(|e| {
            tracing::error!("DB Error: {:?}", e);
            AppError::AuthError
        })?
        .ok_or(AppError::NotFound(format!("User {} not found", user_id)))?;

    Ok(Json(UserResponse::from(user)))
}
