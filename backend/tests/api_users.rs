use std::usize;

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

#[sqlx::test]
async fn test_login_success(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let email = common::random_email();
    let password = "password123";

    let signup_payload = json!({
        "name": "Login User",
        "email": email,
        "password": password
    });

    let signup_req = Request::builder()
        .uri("/signup")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(signup_payload.to_string()))
        .unwrap();
    // appをクローンして使用（RouterはCloneコストが低い）
    let _ = app.clone().oneshot(signup_req).await.unwrap();

    // Act
    let login_payload = json!({
        "email": email,
        "password": password
    });

    let login_req = Request::builder()
        .uri("/login")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(login_payload.to_string()))
        .unwrap();

    let response = app.oneshot(login_req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(body["token"].is_string());
    let token = body["token"].as_str().unwrap();
    assert!(token.len() > 20);
}

#[sqlx::test]
async fn test_login_fail_wrong_password(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool);
    let email = common::random_email();

    let signup_payload = json!({
        "name": "Test User",
        "email": email,
        "password": "correct_password"
    });
    let signup_req = Request::builder()
        .uri("/signup")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(signup_payload.to_string()))
        .unwrap();

    let _ = app.clone().oneshot(signup_req).await.unwrap();

    // Act
    let login_payload = json!({
        "email": email,
        "password": "wrong_password"
    });
    let login_req = Request::builder()
        .uri("/login")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(login_payload.to_string()))
        .unwrap();

    let response = app.oneshot(login_req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
