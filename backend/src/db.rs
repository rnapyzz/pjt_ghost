use sqlx::{Pool, Postgres, migrate::MigrateError, postgres::PgPoolOptions};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub async fn migrate_run(pool: &DbPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}
