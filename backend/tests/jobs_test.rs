use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ghost::create_app;
use ghost::domain::jobs::BusinessModel;
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_job(pool: PgPool) {
    let user_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user_id,
        "Test User"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO projects (id, name, owner_id) VALUES ($1, $2, $3)",
        project_id,
        "Test Project",
        user_id
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = create_app(pool.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}/jobs", project_id))
                .method("POST")
                .header("Content-Type", "application/json")
                .header("x-user-id", user_id.to_string())
                .body(Body::from(
                    r#"{
                    "name": "Test Job",
                    "description": "This is test Job",
                    "business_model": "media"
                }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let saved_job = sqlx::query!(
        r#"SELECT name, business_model as "business_model: ghost::domain::jobs::BusinessModel"
        FROM jobs WHERE name = $1"#,
        "Test Job"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch job");

    assert_eq!(saved_job.name, "Test Job");
    assert_eq!(saved_job.business_model, BusinessModel::Media);
}

#[sqlx::test]
async fn test_list_jobs(pool: PgPool) {
    let user_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();
    let another_project_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user_id,
        "Test User"
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO projects(id, name, owner_id) VALUES ($1, $2, $3)",
        project_id,
        "Test Project 1",
        user_id
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO projects (id, name, owner_id) VALUES ($1, $2, $3)",
        another_project_id,
        "Test Project 2",
        user_id
    )
    .execute(&pool)
    .await
    .unwrap();

    // ジョブを作る（Project1は2つ、Project2は1つ作っておく）
    sqlx::query!(
        "INSERT INTO jobs (project_id, name, business_model) VALUES ($1, $2, $3), ($1, $4, $5)",
        project_id,
        "Job A",
        BusinessModel::Ses as BusinessModel,
        "Job B",
        BusinessModel::Media as BusinessModel
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO jobs (project_id, name, business_model) VALUES ($1, $2, $3)",
        another_project_id,
        "Job C",
        BusinessModel::Internal as BusinessModel,
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = create_app(pool.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}/jobs", project_id))
                .method("GET")
                .header("Content-Type", "application/json")
                .header("x-user-id", user_id.to_string())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.collect().await.unwrap().to_bytes();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
    let jobs = body_json.as_array().expect("Expedted JSON array");

    assert_eq!(jobs.len(), 2);

    let job_names: Vec<&str> = jobs.iter().map(|j| j["name"].as_str().unwrap()).collect();
    assert!(job_names.contains(&"Job A"));
    assert!(job_names.contains(&"Job B"));
}
