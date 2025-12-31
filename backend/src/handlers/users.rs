use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{AppState, domain::users::User};

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let user = state
        .user_repository
        .create(payload.name)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}
