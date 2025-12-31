use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, domain::projects::Project};

#[derive(Deserialize, Debug)]
pub struct CreateProjectPayload {
    pub name: String,
    pub description: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, String)> {
    let user_id_value = headers.get("x-user-id").ok_or((
        StatusCode::UNAUTHORIZED,
        "x-user-id header required".to_string(),
    ))?;

    let user_id_str = user_id_value
        .to_str()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid header value".to_string()))?;

    let user_id = Uuid::parse_str(user_id_str)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let project = state
        .project_repository
        .create(payload.name, payload.description, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create project: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, Json(project)))
}

pub async fn list_projects(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Project>>, (StatusCode, String)> {
    let user_id_value = headers.get("x-user-id").ok_or((
        StatusCode::UNAUTHORIZED,
        "x-user-id header required".to_string(),
    ))?;

    let user_id_str = user_id_value
        .to_str()
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid header value".to_string()))?;

    let user_id = Uuid::parse_str(user_id_str)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID format".to_string()))?;

    let projects = state.project_repository.list(user_id).await.map_err(|e| {
        tracing::error!("Failed to list projects: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(projects))
}
