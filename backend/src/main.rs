use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    net::{Ipv4Addr, SocketAddrV4},
    time::Duration,
};
use tokio::net::TcpListener;

struct Config {
    database_url: String,
    host: String,
    port: u16,
}

impl Config {
    fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = Config::from_env()?;

    tracing::info!("Connecting to database ...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await?;
    tracing::info!("Database connecting establised");

    let app = Router::new()
        .route("/", get(|| async { "Ghost API v2" }))
        .with_state(pool);

    let addr = SocketAddrV4::new(
        config.host.parse().unwrap_or(Ipv4Addr::new(0, 0, 0, 0)),
        config.port,
    );
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
