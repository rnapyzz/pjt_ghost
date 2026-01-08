use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ghost::create_app;
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::PgPool;
use tower::util::ServiceExt;

#[sqlx::test]
async fn test_create_user(pool: PgPool) {
    let app = create_app(pool.clone());

    let payload = r#"
    {
        "name": "Test User",
        "email": "test@example.com",
        "password": "password124"
    }
    "#;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/signup")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert!(body["id"].is_string());
    assert_eq!(body["name"], "Test User");
    assert_eq!(body["email"], "test@example.com");

    assert!(body["password_hash"].is_null());
    assert!(body["password"].is_null());

    let saved_user = sqlx::query!(
        "SELECT name, email, password_hash, role FROM users WHERE name = $1",
        "Test User"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch user from DB");

    assert_eq!(saved_user.name, "Test User");
    assert_eq!(saved_user.email, "test@example.com");
    assert_eq!(saved_user.role, "member");

    assert_ne!(saved_user.password_hash, "password123");
    assert!(saved_user.password_hash.starts_with("$argon2"));
}
