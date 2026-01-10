use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::Utc;
use ghost::domain::jobs::BusinessModel;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

mod common;

#[sqlx::test]
async fn test_create_job_success(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, user_id) = common::create_user_and_get_token(&pool).await;

    let project_id = common::create_test_project(&pool, user_id).await;

    let payload = json!({
        "name": "Test New Job",
        "description": "Test New Job Description",
        "business_model": "media"
    });

    // Act
    let req = Request::builder()
        .uri(format!("/projects/{}/jobs", project_id))
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["name"], "Test New Job");
    assert_eq!(body["description"], "Test New Job Description");
    assert_eq!(body["project_id"], project_id.to_string());
}

#[sqlx::test]
async fn test_list_jobs(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, user_id) = common::create_user_and_get_token(&pool).await;

    let project_id = common::create_test_project(&pool, user_id).await;

    let job_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO jobs (id, project_id, name, description, business_model, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        job_id,
        project_id,
        "Test Job",
        "Test Job Description",
        BusinessModel::Media as BusinessModel,
        Utc::now(),
        Utc::now(),
    )
    .execute(&pool)
    .await
    .unwrap();

    // Act
    let req = Request::builder()
        .uri(format!("/projects/{}/jobs", project_id))
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(body.is_array());
    let list = body.as_array().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0]["name"], "Test Job")
}

#[sqlx::test]
async fn test_get_job(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, user_id) = common::create_user_and_get_token(&pool).await;

    let project_id = common::create_test_project(&pool, user_id).await;

    let job_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO jobs (id, project_id, name, description, business_model, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        job_id,
        project_id,
        "Test Job",
        "Test Job Description (for Get)",
        BusinessModel::Media as BusinessModel,
        Utc::now(),
        Utc::now(),
    )
    .execute(&pool)
    .await
    .unwrap();

    // Act
    let req = Request::builder()
        .uri(format!("/projects/{}/jobs/{}", project_id, job_id))
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["id"], job_id.to_string());
    assert_eq!(body["name"], "Test Job");
    assert_eq!(body["description"], "Test Job Description (for Get)");

    let expected_model = serde_json::to_value(BusinessModel::Media).unwrap();
    assert_eq!(body["business_model"], expected_model);
}
