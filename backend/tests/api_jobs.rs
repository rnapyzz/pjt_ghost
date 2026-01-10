use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::Utc;
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

    let project_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO projects (id, name, description, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        project_id,
        "Test Project",
        "Test Project Description",
        user_id,
        Utc::now(),
        Utc::now(),
    )
    .execute(&pool)
    .await
    .unwrap();

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
