use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub mod domain;
pub mod handlers;
pub mod repositories;

use crate::repositories::users::UserRepositoryImpl;
use crate::{
    domain::{projects::ProjectRepository, users::UserRepository},
    repositories::projects::ProjectRepositoryImpl,
};

// DIコンテナ
#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub project_repository: Arc<dyn ProjectRepository>,
}

// ルーターを作る関数
pub fn create_app(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let user_repository = UserRepositoryImpl::new(pool.clone());
    let project_repository = ProjectRepositoryImpl::new(pool.clone());

    let state = AppState {
        user_repository: Arc::new(user_repository),
        project_repository: Arc::new(project_repository),
    };

    Router::new()
        .route("/", get(root))
        .route("/users", post(handlers::users::create_user))
        .route(
            "/projects",
            post(handlers::projects::create_project).get(handlers::projects::list_projects),
        )
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

// サーバーを起動する関数
pub async fn run(listener: TcpListener, pool: PgPool) -> Result<(), std::io::Error> {
    let app = create_app(pool);
    axum::serve(listener, app).await
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
