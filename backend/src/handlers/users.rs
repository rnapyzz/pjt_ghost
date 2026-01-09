use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, encode};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    domain::users::{Claims, User},
};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

/// 新規ユーザー登録用ハンドラー
///
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process password".to_string(),
            )
        })?
        .to_string();

    let user = state
        .user_repository
        .create(payload.name, payload.email, password_hash)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// ログイン用ハンドラー
///
#[derive(Validate, Debug, Deserialize)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    if let Err(e) = payload.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }

    let user = state
        .user_repository
        .find_by_email(&payload.email)
        .await
        .map_err(|e| {
            tracing::error!("Database error during login: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
        })?
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ))?;

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        tracing::error!("Hash parsing error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Auth error".to_string())
    })?;

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ));
    }

    let now = Utc::now();
    let expire = now + Duration::hours(24);

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expire.timestamp() as usize,
        iat: now.timestamp() as usize,
        name: user.name,
    };

    let token = encode(&Header::default(), &claims, &state.jwt_encoding_key).map_err(|e| {
        tracing::error!("Token creation error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Token creation failed".to_string(),
        )
    })?;

    Ok(Json(LoginResponse { token }))
}
