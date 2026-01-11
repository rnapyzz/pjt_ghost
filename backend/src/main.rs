use axum::{Router, routing::get};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;

use ghost_api::{config, db};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = config::Config::from_env()?;

    tracing::info!("Connecting to database ...");
    let pool = db::create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    tracing::info!("Running migrations...");
    db::migrate_run(&pool).await?;
    tracing::info!("Migration complete");

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
