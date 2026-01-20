use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use slug::slugify;
use uuid::Uuid;

use crate::{
    AppState,
    domains::segment::{CreateSegmentParam, Segment, SegmentUiConfig},
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateSegmentRequest {
    slug: Option<String>,
    name: String,
    description: Option<String>,
    ui_config: Option<SegmentUiConfig>,
}

pub async fn list_segment(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Segment>>> {
    let segments = state.segment_repository.find_all().await?;
    Ok(Json(segments))
}

pub async fn create_segment(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateSegmentRequest>,
) -> Result<(StatusCode, Json<Segment>)> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let slug = match payload.slug {
        Some(s) if !s.trim().is_empty() => s,
        _ => slugify(&payload.name),
    };

    let param = CreateSegmentParam {
        name: payload.name,
        slug,
        description: payload.description,
        ui_config: payload.ui_config.unwrap_or_default(),
        created_by: user_id,
    };

    let segment = state.segment_repository.create(param).await?;

    Ok((StatusCode::CREATED, Json(segment)))
}
