use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
async fn test_create_project_success(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, _user_id) = common::create_user_and_get_token(&pool).await;

    let payload = json!({
        "name": "New Test Project",
        "description": "This is a new project for integration test."
    });

    // Act
    let req = Request::builder()
        .uri("/projects")
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

    assert_eq!(body["name"], "New Test Project");
    assert_eq!(
        body["description"],
        "This is a new project for integration test."
    );
    assert!(body["id"].is_string());
}

#[sqlx::test]
async fn test_create_project_unauthorized(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    // トークンを作らない

    let payload = json!({
        "name": "Test Project",
        "description": "This is project by unauthrized user"
    });

    // Act
    let req = Request::builder()
        .uri("/projects")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
