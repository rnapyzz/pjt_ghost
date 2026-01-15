use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domains::theme::{CreateThemeParam, Theme, UpdateThemeParam},
    error::{AppError, Result},
    extractors::AuthUser,
};

/// テーマ作成リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct CreateThemeRequest {
    title: String,
    description: Option<String>,
}

/// テーマ更新リクエスト
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateThemeRequest {
    title: Option<String>,
    description: Option<String>,
    is_active: Option<bool>,
}

/// 一覧取得 (GET /themes)
pub async fn list_themes(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Theme>>> {
    let themes = state.theme_repository.find_all().await?;

    Ok(Json(themes))
}

/// 新規作成 (POST /themes)
pub async fn create_theme(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateThemeRequest>,
) -> Result<(StatusCode, Json<Theme>)> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;
    let param = CreateThemeParam {
        title: payload.title,
        description: payload.description,
        created_by: user_id,
    };

    let theme = state.theme_repository.create(param).await?;

    Ok((StatusCode::CREATED, Json(theme)))
}

/// 詳細取得 (GET /themes/{id})
pub async fn get_theme(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<Json<Theme>> {
    let theme = state
        .theme_repository
        .find_by_id(id)
        .await?
        .ok_or(AppError::NotFound(format!("Theme {} not found", id)))?;

    Ok(Json(theme))
}

/// 更新 (PATCH /themes/{id})
pub async fn update_theme(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateThemeRequest>,
) -> Result<Json<Theme>> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let param = UpdateThemeParam {
        title: payload.title,
        description: payload.description,
        is_active: payload.is_active,
        updated_by: user_id,
    };

    let update_theme = state.theme_repository.update(id, param).await?;

    Ok(Json(update_theme))
}

/// 削除 (DELETE /themes/{id})
pub async fn delete_theme(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<StatusCode> {
    state.theme_repository.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
