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

#[sqlx::test]
async fn test_update_job(pool: PgPool) {
    let user_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();
    let job_id = Uuid::new_v4();

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

    sqlx::query!(
        "INSERT INTO jobs (id, project_id, name, description, business_model) VALUES ($1, $2, $3, $4, $5)",
        job_id, project_id, "Test Job", "This is the description",
        BusinessModel::Media as BusinessModel 
    ).execute(&pool).await.unwrap();

    let app = create_app(pool.clone());

    let response = app.clone()
        .oneshot(
            Request::builder()
            .uri(format!("/projects/{}/jobs/{}", project_id, job_id))
            .method("PATCH")
            .header("Content-Type", "application/json")
            .header("x-user-id", user_id.to_string())
            .body(Body::from(
                r#"{
                    "name": "New Job Name",
                    "business_model": "saas"
                }"#
            )).unwrap(),
        ).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let updated_job = sqlx::query!(
        r#"
            SELECT name, description, business_model as "business_model: ghost::domain::jobs::BusinessModel"
            FROM jobs
            WHERE id = $1
        "#, job_id
    ).fetch_one(&pool).await.unwrap();

    // name と　　business_model は変更してある
    assert_eq!(updated_job.name, "New Job Name");
    assert_eq!(updated_job.business_model, BusinessModel::Saas);
    // description は変更してない
    assert_eq!(updated_job.description, "This is the description");

    // 間違ったproject_idを指定した場合 (失敗)
    let wrong_project_id = Uuid::new_v4();
    let response_err = app
        .oneshot(
            Request::builder()
            .uri(format!("/projects/{}/jobs/{}", wrong_project_id, job_id))
            .method("PATCH")
            .header("Content-Type", "application/json")
            .header("x-user-id", user_id.to_string())
            .body(Body::from(r#"{ "name": "" }"#)).unwrap()
        ).await.unwrap();
    
    assert_eq!(response_err.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test]
async fn test_delete_job(pool:PgPool) {
    let user_id = Uuid::new_v4();
    let project_id = Uuid::new_v4();
    let job_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user_id, "Test User"
    ).execute(&pool).await.unwrap();

    sqlx::query!(
        "INSERT INTO projects (id, name, owner_id) VALUES ($1, $2, $3)",
        project_id, "Test Project", user_id,
    ).execute(&pool).await.unwrap();

    sqlx::query!(
        "INSERT INTO jobs (id, project_id, name, business_model) VALUES ($1, $2, $3, $4)",
        job_id, project_id, "Test Job", BusinessModel::Internal as BusinessModel
    ).execute(&pool).await.unwrap();

    let app = create_app(pool.clone());

    let response = app
    .oneshot(
        Request::builder()
        .uri(format!("/projects/{}/jobs/{}", project_id, job_id))
        .method("DELETE")
        .header("x-user-id", user_id.to_string())
        .body(Body::empty()).unwrap()
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let count = sqlx::query!(
        "SELECT count(*) as count FROM jobs WHERE id = $1", job_id
    ).fetch_one(&pool).await.unwrap();

    assert_eq!(count.count, Some(0));
}