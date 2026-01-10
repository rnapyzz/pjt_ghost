use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

mod common;

#[sqlx::test]
async fn test_signup_success(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool);
    let email = common::random_email();

    let payload = json!({
        "name": "Test User",
        "email": email,
        "password": "password123"
    });

    // Act
    let req = Request::builder()
        .uri("/signup")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["name"], "Test User");
    assert_eq!(body["email"], email);
    assert!(body.get("password").is_none());
    assert!(body.get("password_hash").is_none());
}
