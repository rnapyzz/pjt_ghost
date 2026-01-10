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

#[sqlx::test]
async fn test_list_projects(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, user_id) = common::create_user_and_get_token(&pool).await;

    sqlx::query!(
        "INSERT INTO projects (id, name, description, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        Uuid::new_v4(),
        "Existing Project",
        "Description ... ",
        user_id,
        Utc::now(),
        Utc::now(),
    )
    .execute(&pool)
    .await
    .unwrap();

    // Act
    let req = Request::builder()
        .uri("/projects")
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
    assert_eq!(list[0]["name"], "Existing Project");
}

#[sqlx::test]
async fn test_update_project_success(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, user_id) = common::create_user_and_get_token(&pool).await;

    let project_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO projects (id, name, description, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        project_id,
        "Old Name",
        "Old Description",
        user_id,
        Utc::now(),
        Utc::now()
    )
    .execute(&pool)
    .await
    .unwrap();

    let payload = json!({
        "name": "Updated Name",
        "description": "Updated Description"
    });

    // Act
    let req = Request::builder()
        .uri(format!("/projects/{}", project_id))
        .method("PATCH")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(payload.to_string()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(body["name"], "Updated Name");
    assert_eq!(body["description"], "Updated Description");
}

#[sqlx::test]
async fn test_delete_project_success(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, user_id) = common::create_user_and_get_token(&pool).await;

    let project_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO projects (id, name, description, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        project_id,
        "Project to be deleted",
        "some description",
        user_id,
        Utc::now(),
        Utc::now()
    )
    .execute(&pool)
    .await
    .unwrap();

    // Act
    let req = Request::builder()
        .uri(format!("/projects/{}", project_id))
        .method("DELETE")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let count = sqlx::query!(
        "SELECT count(*) as count FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .count;

    assert_eq!(count, Some(0));
}

#[sqlx::test]
async fn test_delete_project_forbidden_for_others(pool: PgPool) {
    // Arrange
    let app = common::setup_app(pool.clone());
    let (token, _user_id) = common::create_user_and_get_token(&pool).await;

    let (_, other_user_id) = common::create_user_and_get_token(&pool).await;
    let project_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO projects (id, name, description, owner_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        project_id,
        "Project to be deleted",
        "some description",
        other_user_id,
        Utc::now(),
        Utc::now()
    )
    .execute(&pool)
    .await
    .unwrap();

    // Act
    let req = Request::builder()
        .uri(format!("/projects/{}", project_id))
        .method("DELETE")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let count = sqlx::query!(
        "SELECT count(*) as count FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .count;

    assert_eq!(count, Some(1));
}
