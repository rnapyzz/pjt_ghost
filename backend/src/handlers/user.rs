use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{
    AppState,
    domains::user::{CreateUserParam, User, UserRole},
    error::Result,
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
