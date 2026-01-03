use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    domain::jobs::{BusinessModel, Job},
};

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub description: String,
    pub business_model: BusinessModel,
}

#[derive(Debug, Deserialize)]
pub struct UpdateJobRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub business_model: Option<BusinessModel>,
}

pub async fn create_job(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<CreateJobRequest>,
) -> Result<(StatusCode, Json<Job>), (StatusCode, String)> {
    let job = state
        .job_repository
        .create(
            project_id,
            payload.name,
            payload.description,
            payload.business_model,
        )
        .await
        .map_err(|e| {
            tracing::error!("Failed to create job: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::CREATED, Json(job)))
}

pub async fn list_jobs(
    Path(project_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Job>>), (StatusCode, String)> {
    let jobs = state
        .job_repository
        .find_by_project_id(project_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list jobs: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::OK, Json(jobs)))
}

pub async fn update_job(
    Path((project_id, job_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateJobRequest>,
) -> Result<(StatusCode, Json<Job>), (StatusCode, String)> {
    let job = state
        .job_repository
        .update(
            job_id,
            project_id,
            payload.name,
            payload.description,
            payload.business_model,
        )
        .await
        .map_err(|e| {
            if let Some(sqlx::Error::RowNotFound) = e.downcast_ref::<sqlx::Error>() {
                return (
                    StatusCode::NOT_FOUND,
                    "Job not found or Project not found".to_string(),
                );
            }
            tracing::error!("Failed to update projects: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok((StatusCode::OK, Json(job)))
}

pub async fn delete_job(
    Path((project_id, job_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let count = state
        .job_repository
        .delete(job_id, project_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete job: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if count == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            "Job not found or Project not found".to_string(),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
