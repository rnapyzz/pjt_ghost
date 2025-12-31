use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ghost::create_app;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

#[sqlx::test]
async fn test_create_project(pool: PgPool) {
    // テストのためのユーザーを作成する
    let user_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user_id,
        "Test Owner"
    )
    .execute(&pool)
    .await
    .expect("Failed to create seed user");

    let app = create_app(pool.clone());

    // リクエストの実行
    let response = app
        .oneshot(
            Request::builder()
                .uri("/projects")
                .method("POST")
                .header("Content-Type", "application/json")
                .header("x-user-id", user_id.to_string())
                .body(Body::from(
                    r#"{"name": "New Test Project","description": "This is a test project"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードのチェック
    assert_eq!(response.status(), StatusCode::CREATED);

    let saved_project = sqlx::query!(
        "SELECT name, owner_id FROM projects WHERE name = $1",
        "New Test Project"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch project");

    // 作成された内容のチェック
    assert_eq!(saved_project.name, "New Test Project");
    assert_eq!(saved_project.owner_id, user_id);
}
