use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch, post},
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

use crate::{
    domain::{jobs::JobRepository, projects::ProjectRepository, users::UserRepository},
    repositories::{
        jobs::JobRepositoryImpl, projects::ProjectRepositoryImpl, users::UserRepositoryImpl,
    },
};

// DIコンテナ
#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub project_repository: Arc<dyn ProjectRepository>,
    pub job_repository: Arc<dyn JobRepository>,
}

// ルーターを作る関数
pub fn create_app(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let user_repository = UserRepositoryImpl::new(pool.clone());
    let project_repository = ProjectRepositoryImpl::new(pool.clone());
    let job_repository = JobRepositoryImpl::new(pool.clone());

    let state = AppState {
        user_repository: Arc::new(user_repository),
        project_repository: Arc::new(project_repository),
        job_repository: Arc::new(job_repository),
    };

    Router::new()
        .route("/", get(root))
        .route("/users", post(handlers::users::create_user))
        .route(
            "/projects",
            post(handlers::projects::create_project).get(handlers::projects::list_projects),
        )
        .route(
            "/projects/{id}",
            patch(handlers::projects::update_project).delete(handlers::projects::delete_project),
        )
        .route(
            "/projects/{id}/jobs",
            post(handlers::jobs::create_job).get(handlers::jobs::list_jobs),
        )
        .route(
            "/projects/{id}/jobs/{job_id}",
            patch(handlers::jobs::update_job).delete(handlers::jobs::delete_job),
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
