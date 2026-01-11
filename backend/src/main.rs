use axum::{Router, routing::get};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new().route("/", get(|| async { "Ghost API v2" }));
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3000);
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
