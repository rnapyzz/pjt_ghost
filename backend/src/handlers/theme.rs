use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domains::theme::{CreateThemeParam, Theme},
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateThemeRequest {
    title: String,
    description: Option<String>,
}

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

pub async fn list_themes(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Theme>>> {
    let themes = state.theme_repository.find_all().await?;

    Ok(Json(themes))
}
