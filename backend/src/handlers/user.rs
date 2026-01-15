use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domains::user::{CreateUserParam, User, UserRole},
    error::{AppError, Result},
};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub employee_id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<UserRole>,
}

/// 新規作成 (POST /signup)
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>)> {
    let payload = CreateUserParam {
        employee_id: payload.employee_id,
        username: payload.username,
        name: payload.name,
        email: payload.email,
        password: payload.password,
        role: payload.role,
    };

    let user = state.user_repository.create(payload).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

/// 詳細取得 (GET /users/{id})
pub async fn get_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<User>> {
    let user = state.user_repository.find_by_id(id).await?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFound(format!("User {} not found", id))),
    }
}
