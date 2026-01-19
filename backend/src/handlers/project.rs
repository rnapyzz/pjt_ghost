use axum::{Json, extract::State};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domains::project::{CreateProjectParam, Project, ProjectType},
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub theme_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type", default)]
    pub type_: ProjectType,
    pub attributes: Option<serde_json::Value>,
    pub owner_id: Option<Uuid>,
}

pub async fn list_projects(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Project>>> {
    let projects = state.project_repository.find_all().await?;

    Ok(Json(projects))
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
        project_type: payload.type_,
        attributes: payload.attributes,
        owner_id: payload.owner_id,
        created_by: user_id,
    };

    let project = state.project_repository.create(param).await?;

    Ok(Json(project))
}

