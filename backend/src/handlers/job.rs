use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domains::job::{CreateJobParam, Job, UpdateJobParam},
    error::{AppError, Result},
    extractors::AuthUser,
};

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub service_id: Uuid,
    pub project_id: Option<Uuid>,
    pub theme_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,

    #[serde(default = "default_status")]
    pub status: String,

    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub service_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub theme_id: Option<Uuid>,

    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub owner_id: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

fn default_status() -> String {
    "Draft".to_string()
}

pub async fn list_jobs(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Json<Vec<Job>>> {
    let jobs = state.job_repository.find_all().await?;
    Ok(Json(jobs))
}

pub async fn create_job(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateJobRequest>,
) -> Result<Json<Job>> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let param = CreateJobParam {
        service_id: payload.service_id,
        project_id: payload.project_id,
        theme_id: payload.theme_id,
        title: payload.title,
        description: payload.description,
        status: payload.status,
        owner_id: payload.owner_id,
        created_by: user_id,
    };

    let job = state.job_repository.create(param).await?;

    Ok(Json(job))
}

pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<Json<Job>> {
    let job = state.job_repository.find_by_id(id).await?;

    match job {
        Some(j) => Ok(Json(j)),
        None => Err(AppError::NotFound(format!("Job '{}' not found", id))),
    }
}

// 更新 (PATCH /jobs/{id})
pub async fn update_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    auth_user: AuthUser,
    Json(payload): Json<UpdateJobRequest>,
) -> Result<Json<Job>> {
    let user_id = Uuid::parse_str(&auth_user.claims.sub).map_err(|_| AppError::AuthError)?;

    let param = UpdateJobParam {
        service_id: payload.service_id,
        project_id: payload.project_id,
        theme_id: payload.theme_id,
        title: payload.title,
        description: payload.description,
        status: payload.status,
        owner_id: payload.owner_id,
        updated_by: user_id,
    };

    let update_job = state.job_repository.update(id, param).await?;

    Ok(Json(update_job))
}

// 削除 (DELET /jobs/{id})
pub async fn delete_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<StatusCode> {
    state.job_repository.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
