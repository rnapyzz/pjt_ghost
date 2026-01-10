use axum::Router;
use sqlx::PgPool;

use ghost::create_app;

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
