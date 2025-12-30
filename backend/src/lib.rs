use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    axum::serve(listener, app).await
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
