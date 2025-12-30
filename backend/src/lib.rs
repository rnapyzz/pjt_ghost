use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    axum::serve(listener, app).await
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
