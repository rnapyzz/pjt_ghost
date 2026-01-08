use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
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
    let app = create_app(pool.clone(), "test_secret_key");

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

/// テスト用のユーザーを作成してDBに保存するヘルパー関数
async fn create_user_for_login(pool: &PgPool, email: &str, password: &str) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password in test")
        .to_string();

    sqlx::query!(
        "INSERT INTO users (name, email, password_hash, role) VALUES ($1, $2, $3, 'member')",
        "Login Test User",
        email,
        password_hash
    )
    .execute(pool)
    .await
    .expect("Failed to insert test user");
}

#[sqlx::test]
async fn test_login(pool: PgPool) {
    let app = create_app(pool.clone(), "test_secret_key");

    let email = "test_login@example.com";
    let password = "testuserpassword123";

    create_user_for_login(&pool, email, password).await;

    let login_payload = format!(
        r#"{{
        "email": "{}",
        "password": "{}"
    }}"#,
        email, password
    );

    let login_req = Request::builder()
        .uri("/login")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(login_payload))
        .unwrap();

    let login_res = app.clone().oneshot(login_req).await.unwrap();

    assert_eq!(login_res.status(), StatusCode::OK);

    let body_bytes = login_res.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(body["token"].is_string());

    let token = body["token"].as_str().unwrap();
    println!("Got Token: {}", token);

    assert!(token.len() > 30);

    let wrong_payload = r#"{
        "email": "test_login@example.com",
        "password": "wrongpassword"
    }"#;

    let wrong_req = Request::builder()
        .uri("/login")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(wrong_payload))
        .unwrap();

    let wrong_res = app.oneshot(wrong_req).await.unwrap();

    assert_eq!(wrong_res.status(), StatusCode::UNAUTHORIZED);
}
