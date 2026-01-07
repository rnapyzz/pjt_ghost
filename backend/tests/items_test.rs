use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use ghost::{
    create_app,
    domain::{
        items::{Account, Item},
        jobs::BusinessModel,
    },
};
use http_body_util::BodyExt;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

#[sqlx::test]
async fn test_list_accounts_and_item_types(pool: PgPool) {
    let app = create_app(pool.clone());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/accounts")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.collect().await.unwrap().to_bytes();
    let accounts: Vec<Account> = serde_json::from_slice(&body_bytes).unwrap();

    assert!(!accounts.is_empty());

    let first_account = &accounts[0];

    let uri = format!("/item-types?account_id={}", first_account.id);
    let response_item_types = app
        .oneshot(
            Request::builder()
                .uri(uri)
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_item_types.status(), StatusCode::OK);

    let body_bytes = response_item_types.collect().await.unwrap().to_bytes();
    let item_types: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

    assert!(item_types.as_array().unwrap().len() > 0);
}

// 今回は保守コストを考えて、createとlistをセットで行うテストにしておく
#[sqlx::test]
async fn test_create_item_and_list_items(pool: PgPool) {
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
        user_id,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query!(
        "INSERT INTO jobs (id, name, project_id, business_model) VALUES ($1, $2, $3, $4)",
        job_id,
        "Test Job",
        project_id,
        BusinessModel::Media as BusinessModel
    )
    .execute(&pool)
    .await
    .unwrap();

    // 適当なItemTypeを一つ取得
    let item_type = sqlx::query!("SELECT id FROM item_types LIMIT 1")
        .fetch_one(&pool)
        .await
        .expect("Seed data not found");

    let app = create_app(pool.clone());

    let response_create = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}/jobs/{}/items", project_id, job_id))
                .method("POST")
                .header("Content-Type", "application/json")
                .header("x-user-id", user_id.to_string())
                .body(Body::from(format!(
                    r#"{{
                        "item_type_id": "{}",
                        "name": "Something Cost",
                        "description": "Random Item for Test",
                        "assignee_id": null,
                        "entries": [
                            {{ "date": "2026-01-01", "amount": 1000000 }},
                            {{ "date": "2026-02-01", "amount": 2000000 }}
                        ]
                    }}"#,
                    item_type.id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_create.status(), StatusCode::CREATED);

    let response_list = app
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}/jobs/{}/items", project_id, job_id))
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response_list.status(), StatusCode::OK);

    let body_bytes = response_list.collect().await.unwrap().to_bytes();
    let items: Vec<Item> = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(items.len(), 1);

    let item = &items[0];

    assert_eq!(item.name, "Something Cost");
    assert_eq!(item.entries.len(), 2);

    let total: i64 = item.entries.iter().map(|e| e.amount).sum();
    assert_eq!(total, 3_000_000);
}

#[sqlx::test]
async fn test_lifecycle_create_upadte_delete(pool: PgPool) {
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
        "INSERT INTO jobs (id, name, project_id, business_model) VALUES ($1, $2, $3, $4)",
        job_id,
        "Test Job",
        project_id,
        BusinessModel::Media as BusinessModel
    )
    .execute(&pool)
    .await
    .unwrap();

    let item_type = sqlx::query!("SELECT id FROM item_types LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    let app = create_app(pool.clone());

    // create
    let res_create = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/projects/{}/jobs/{}/items", project_id, job_id))
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                    "item_type_id": "{}",
                    "name": "New Item",
                    "entries": [
                        {{ "date": "2026-01-01", "amount": 1000 }}
                    ]
                    }}"#,
                    item_type.id
                )))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_create.status(), StatusCode::CREATED);

    let created_item: Item =
        serde_json::from_slice(&res_create.collect().await.unwrap().to_bytes()).unwrap();
    let item_id = created_item.id;

    // update
    let res_update = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/projects/{}/jobs/{}/items/{}",
                    project_id, job_id, item_id
                ))
                .method("PATCH")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{
                "name": "Updated Name",
                "entries": [{
                    "date": "2026-02-01", "amount": 2000
                }]
        }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_update.status(), StatusCode::OK);

    let updated_item: Item =
        serde_json::from_slice(&res_update.collect().await.unwrap().to_bytes()).unwrap();

    assert_eq!(updated_item.name, "Updated Name");
    assert_eq!(updated_item.entries.len(), 1);
    assert_eq!(updated_item.entries[0].amount, 2000);

    let res_delete = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/projects/{}/jobs/{}/items/{}",
                    project_id, job_id, item_id
                ))
                .method("DELETE")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res_delete.status(), StatusCode::NO_CONTENT);

    let item_count = sqlx::query!("SELECT count(*) as c FROM items WHERE id = $1", item_id)
        .fetch_one(&pool)
        .await
        .unwrap()
        .c;
    assert_eq!(item_count, Some(0));

    let entry_count = sqlx::query!("SELECT count(*) as c FROM entries WHERE id = $1", item_id)
        .fetch_one(&pool)
        .await
        .unwrap()
        .c;
    assert_eq!(entry_count, Some(0));
}
