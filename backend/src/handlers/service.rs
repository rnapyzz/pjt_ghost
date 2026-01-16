use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use slug::slugify;
use uuid::Uuid;

use crate::{
    AppState,
    domains::service::{CreateServiceParam, Service},
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateServiceRequest {
    pub name: String,
    pub slug: Option<String>,
    pub owner_id: Option<Uuid>,
}

pub async fn list_service(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Service>>> {
    let services = state.service_repository.find_all().await?;

    Ok(Json(services))
}

pub async fn create_service(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateServiceRequest>,
) -> Result<(StatusCode, Json<Service>)> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let slug = match payload.slug {
        Some(s) if !s.trim().is_empty() => s,
        _ => slugify(&payload.name),
    };

    let param = CreateServiceParam {
        name: payload.name,
        slug: slug,
        owner_id: payload.owner_id,
        created_by: user_id,
    };

    let service = state.service_repository.create(param).await?;

    Ok((StatusCode::CREATED, Json(service)))
}

pub async fn get_service(
    State(state): State<AppState>,
    Path(identifier): Path<String>,
    _auth_user: AuthUser,
) -> Result<Json<Service>> {
    let service = if let Ok(uuid) = Uuid::parse_str(&identifier) {
        state.service_repository.find_by_id(uuid).await?
    } else {
        state.service_repository.find_by_slug(&identifier).await?
    };

    let service = service.ok_or(AppError::NotFound(format!(
        "Service '{}' not found",
        identifier
    )))?;

    Ok(Json(service))
}
