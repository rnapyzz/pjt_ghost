use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domains::project::{CreateProjectParam, Project, ProjectType, UpdateProjectParam},
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub theme_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Option<serde_json::Value>,
    #[serde(rename = "type", default)]
    pub type_: ProjectType,
    pub target_market: Option<String>,
    pub value_prop: Option<String>,
    pub target_client: Option<String>,
    pub kpis: Option<String>,
    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub theme_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub attributes: Option<serde_json::Value>,

    #[serde(rename = "type")]
    pub type_: Option<ProjectType>,
    pub target_market: Option<String>,
    pub value_prop: Option<String>,
    pub target_client: Option<String>,
    pub kpis: Option<String>,

    pub is_active: Option<bool>,
    pub owner_id: Option<Uuid>,
}

pub async fn list_projects(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Project>>> {
    let projects = state.project_repository.find_all().await?;

    Ok(Json(projects))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<Json<Project>> {
    let project = state
        .project_repository
        .find_by_id(id)
        .await?
        .ok_or(AppError::NotFound(format!("Project {} not found", id)))?;

    Ok(Json(project))
}

pub async fn create_project(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateProjectRequest>,
) -> Result<Json<Project>> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let param = CreateProjectParam {
        theme_id: payload.theme_id,
        name: payload.name,
        description: payload.description,
        attributes: payload.attributes,
        project_type: payload.type_,
        target_market: payload.target_market,
        value_prop: payload.value_prop,
        target_client: payload.target_client,
        kpis: payload.kpis,
        owner_id: payload.owner_id,
        created_by: user_id,
    };

    let project = state.project_repository.create(param).await?;

    Ok(Json(project))
}

pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateProjectRequest>,
) -> Result<Json<Project>> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let param = UpdateProjectParam {
        theme_id: payload.theme_id,
        name: payload.name,
        description: payload.description,
        attributes: payload.attributes,
        project_type: payload.type_,
        target_market: payload.target_market,
        value_prop: payload.value_prop,
        target_client: payload.target_client,
        kpis: payload.kpis,
        is_active: payload.is_active,
        owner_id: payload.owner_id,
        updated_by: user_id,
    };

    let project = state.project_repository.update(id, param).await?;

    Ok(Json(project))
}

pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<StatusCode> {
    state.project_repository.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
