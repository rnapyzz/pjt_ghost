use std::sync::Arc;

use axum::{
    Router, middleware,
    routing::{get, patch, post, put},
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub mod domain;
pub mod handlers;
pub mod middlewares;
pub mod repositories;

use crate::{
    domain::{
        items::ItemRepository, jobs::JobRepository, projects::ProjectRepository,
        users::UserRepository,
    },
    middlewares::auth_middleware,
    repositories::{
        items::ItemRepositoryImpl, jobs::JobRepositoryImpl, projects::ProjectRepositoryImpl,
        users::UserRepositoryImpl,
    },
};

// DIコンテナ
#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub project_repository: Arc<dyn ProjectRepository>,
    pub job_repository: Arc<dyn JobRepository>,
    pub item_repository: Arc<dyn ItemRepository>,
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey,
}

// ルーターを作る関数
pub fn create_app(pool: PgPool, jwt_secret: &str) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let user_repository = UserRepositoryImpl::new(pool.clone());
    let project_repository = ProjectRepositoryImpl::new(pool.clone());
    let job_repository = JobRepositoryImpl::new(pool.clone());
    let item_repository = ItemRepositoryImpl::new(pool.clone());

    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

    let state = AppState {
        user_repository: Arc::new(user_repository),
        project_repository: Arc::new(project_repository),
        job_repository: Arc::new(job_repository),
        item_repository: Arc::new(item_repository),
        jwt_encoding_key: encoding_key,
        jwt_decoding_key: decoding_key,
    };

    // 認証が必要なルート
    let protected_route = Router::new()
        .route(
            "/projects",
            post(handlers::projects::create_project).get(handlers::projects::list_projects),
        )
        .route(
            "/projects/{id}",
            get(handlers::projects::get_project)
                .patch(handlers::projects::update_project)
                .delete(handlers::projects::delete_project),
        )
        .route(
            "/projects/{id}/jobs",
            post(handlers::jobs::create_job).get(handlers::jobs::list_jobs),
        )
        .route(
            "/projects/{id}/jobs/{job_id}",
            get(handlers::jobs::get_job)
                .patch(handlers::jobs::update_job)
                .delete(handlers::jobs::delete_job),
        )
        .route(
            "/projects/{id}/jobs/{job_id}/export-csv",
            get(handlers::items::export_items_csv),
        )
        .route(
            "/projects/{id}/jobs/{job_id}/items",
            post(handlers::items::create_item).get(handlers::items::list_items),
        )
        .route(
            "/projects/{id}/jobs/{job_id}/items/{item_id}",
            patch(handlers::items::update_item).delete(handlers::items::delete_item),
        )
        .route(
            "/projects/{id}/jobs/{job_id}/items/{item_id}/entries",
            put(handlers::items::update_entries),
        )
        .route("/accounts", get(handlers::items::list_accounts))
        .route("/item-types", get(handlers::items::list_item_types))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    // 全体のルーター (Public Routes + Protected Routes)
    Router::new()
        .route("/", get(root))
        .route("/signup", post(handlers::users::create_user))
        .route("/login", post(handlers::users::login))
        .merge(protected_route)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

// サーバーを起動する関数
pub async fn run(
    listener: TcpListener,
    pool: PgPool,
    jwt_secret: &str,
) -> Result<(), std::io::Error> {
    let app = create_app(pool, jwt_secret);
    axum::serve(listener, app).await
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
