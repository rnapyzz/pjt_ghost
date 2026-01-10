use axum::Router;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

use ghost::{create_app, domain::users::Claims};
use uuid::Uuid;

// テスト用の定数
pub const TEST_JWT_SECRET: &str = "test_secret_key_for_integration_testing";

// アプリケーションをセットアップするヘルパー関数
pub fn setup_app(pool: PgPool) -> Router {
    create_app(pool, TEST_JWT_SECRET)
}

// ランダムなメールアドレスを作成するヘルパー
// 並行してテスト実行のときに、メールアドレスが重複するエラーを防止
pub fn random_email() -> String {
    use uuid::Uuid;
    format!("test_{}@example.com", Uuid::new_v4())
}

// ユーザーを追加して、認証トークンを返す関数
pub async fn create_user_and_get_token(pool: &PgPool) -> (String, Uuid) {
    let email = random_email();
    let user_id = Uuid::new_v4();
    let name = "Test User";

    sqlx::query!(
        "INSERT INTO users (id, name, email, password_hash, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        user_id,
        name,
        email,
        "dummy_hash_for_test",
        Utc::now(),
        Utc::now(),
    )
    .execute(pool)
    .await
    .expect("Failed to insert test user");

    let claims = Claims {
        sub: user_id.to_string(),
        name: name.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(TEST_JWT_SECRET.as_bytes()),
    )
    .expect("Failed to encode token");

    (token, user_id)
}
