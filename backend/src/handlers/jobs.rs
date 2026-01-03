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
