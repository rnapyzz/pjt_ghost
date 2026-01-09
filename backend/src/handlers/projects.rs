use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domain::{projects::Project, users::Claims},
    utils::extract_user_id,
};

#[derive(Deserialize, Debug)]
pub struct CreateProjectPayload {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProjectPayload {
    pub name: String,
    pub description: String,
}

pub async fn create_project(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateProjectPayload>,
) -> Result<(StatusCode, Json<Project>), (StatusCode, String)> {
    let user_id = extract_user_id(&claims)?;

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
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Project>>, (StatusCode, String)> {
    let user_id = extract_user_id(&claims)?;

    let projects = state.project_repository.list(user_id).await.map_err(|e| {
        tracing::error!("Failed to list projects: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok(Json(projects))
}

pub async fn get_project(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Project>, (StatusCode, String)> {
    let project = state
        .project_repository
        .find_by_id(id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find project: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .ok_or((StatusCode::NOT_FOUND, "Project not found".to_string()))?;

    Ok(Json(project))
}

pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProjectPayload>,
) -> Result<Json<Project>, (StatusCode, String)> {
    let user_id = extract_user_id(&claims)?;

    let project = state
        .project_repository
        .update(id, payload.name, payload.description, user_id)
        .await
        .map_err(|e| {
            if let Some(sqlx::Error::RowNotFound) = e.downcast_ref::<sqlx::Error>() {
                return (
                    StatusCode::NOT_FOUND,
                    "Project not found or permission denied".to_string(),
                );
            }
            tracing::error!("Failed to update projects: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(project))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Extension(claims): Extension<Claims>,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = extract_user_id(&claims)?;

    let count = state
        .project_repository
        .delete(id, user_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete project: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if count == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            "Project not found or permission denied".to_string(),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
