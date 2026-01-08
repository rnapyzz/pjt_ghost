use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

use crate::{AppState, domain::users::User};

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    #[validate(length(min = 1, message = "名前は必須です"))]
    pub name: String,

    #[validate(email(message = "正しいメールアドレス形式ではありません"))]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上必要です"))]
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserReq>,
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
