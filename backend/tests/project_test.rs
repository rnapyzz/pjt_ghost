use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ghost::create_app;
use http_body_util::BodyExt;
use serde_json::Value;
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

#[sqlx::test]
async fn test_list_projects(pool: PgPool) {
    // テスト用のユーザーを2つ作る
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();

    // ユーザー作成処理
    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user1_id,
        "Test User1"
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2)",
        user2_id,
        "Test User2"
    )
    .execute(&pool)
    .await
    .unwrap();

    // プロジェクト作成処理
    sqlx::query!(
        "INSERT INTO projects (name, owner_id) VALUES ($1, $2)",
        "Project A1",
        user1_id
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query!(
        "INSERT INTO projects (name, owner_id) VALUES ($1, $2)",
        "Project A2",
        user1_id
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query!(
        "INSERT INTO projects (name, owner_id) VALUES ($1, $2)",
        "Project B",
        user2_id
    )
    .execute(&pool)
    .await
    .unwrap();

    // アプリの起動
    let app = create_app(pool.clone());

    // User1としてリクエストを実施
    let response = app
        .oneshot(
            Request::builder()
                .uri("/projects")
                .method("GET")
                .header("x-user-id", user1_id.to_string())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // ステータスコードの検証
    assert_eq!(response.status(), StatusCode::OK);

    // 取得したデータを解析
    let body_bytes = response.collect().await.unwrap().to_bytes();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

    let projects = body_json.as_array().expect("Expected JSON array");

    // User1のプロジェクトの件数の検証
    assert_eq!(projects.len(), 2);

    // Projectデータを確認
    let project_names: Vec<&str> = projects
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();

    assert!(project_names.contains(&"Project A1"));
    assert!(project_names.contains(&"Project A2"));
    assert!(!project_names.contains(&"Project B1"));
}

#[sqlx::test]
async fn test_update_project(pool: PgPool) {
    // テスト用のユーザーを2つ作成
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2), ($3, $4)",
        user1_id,
        "User 1",
        user2_id,
        "User 2"
    )
    .execute(&pool)
    .await
    .unwrap();

    // テスト用のプロジェクトを作成
    let project_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO projects (id, name, owner_id) VALUES ($1, $2, $3)",
        project_id,
        "Original Name",
        user1_id
    )
    .execute(&pool)
    .await
    .unwrap();

    // テストケース1: projectのownerが更新する場合 (成功)
    let app = create_app(pool.clone());
    let response_ok = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}", project_id))
                .method("PATCH")
                .header("Content-Type", "application/json")
                .header("x-user-id", user1_id.to_string())
                .body(Body::from(
                    r#"{
                        "name": "Updated Name",
                        "description": "Updated Description"
                }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_ok.status(), StatusCode::OK);

    let updated_project = sqlx::query!("SELECT name FROM projects WHERE id = $1", project_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(updated_project.name, "Updated Name");

    // テストケース2: projectのownerでない場合 (失敗)
    let response_err = app
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}", project_id))
                .method("PATCH")
                .header("Content-Type", "application/json")
                .header("x-user-id", user2_id.to_string())
                .body(Body::from(
                    r#"{
                        "name": "Hacked Name",
                        "description": "Hacked Description"                    
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_err.status(), StatusCode::NOT_FOUND);

    let protected_project = sqlx::query!("SELECT name FROM projects WHERE id = $1", project_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(protected_project.name, "Updated Name");
}

#[sqlx::test]
async fn test_delete_project(pool: PgPool) {
    // テスト用のユーザーを2つ作成
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO users (id, name) VALUES ($1, $2), ($3, $4)",
        user1_id,
        "User 1",
        user2_id,
        "User 2",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Projectを作成 (ownerはUser 1)
    let project_id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO projects (id, name, owner_id) VALUES ($1, $2, $3)",
        project_id,
        "Project to Delete",
        user1_id
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = create_app(pool.clone());

    // テストケース1: owner以外が削除しようとする場合 (NOT FOUND)
    let response_err = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}", project_id))
                .method("DELETE")
                .header("x-user-id", user2_id.to_string())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_err.status(), StatusCode::NOT_FOUND);

    // 失敗後にデータが残っていることの確認
    let count = sqlx::query!(
        "SELECT count(*) as count FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count.count, Some(1));

    // テストケース2: ownerが削除する (成功)
    let response_ok = app
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}", project_id))
                .method("DELETE")
                .header("x-user-id", user1_id.to_string())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_ok.status(), StatusCode::NO_CONTENT);

    // データが確かに消えていることを確認
    let count_after_delete = sqlx::query!(
        "SELECT count(*) as count FROM projects WHERE id = $1",
        project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count_after_delete.count, Some(0));
}
