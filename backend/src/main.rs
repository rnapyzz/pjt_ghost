use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{delete, get, patch, post},
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use ghost_api::{
    AppState, config, db, handlers,
    repositories::{
        account_item::AccountItemRepositoryImpl, job::JobRepositoryImpl,
        project::ProjectRepositoryImpl, segment::SegmentRepositoryImpl,
        service::ServiceRepositoryImpl, theme::ThemeRepositoryImpl, user::UserRepositoryImpl,
    },
};

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

    let user_repository = UserRepositoryImpl::new(pool.clone());
    let theme_repository = ThemeRepositoryImpl::new(pool.clone());
    let project_repository = ProjectRepositoryImpl::new(pool.clone());
    let segment_repository = SegmentRepositoryImpl::new(pool.clone());
    let service_repository = ServiceRepositoryImpl::new(pool.clone());
    let job_repository = JobRepositoryImpl::new(pool.clone());
    let account_item_repository = AccountItemRepositoryImpl::new(pool.clone());

    let state = AppState {
        user_repository: Arc::new(user_repository),
        theme_repository: Arc::new(theme_repository),
        project_repository: Arc::new(project_repository),
        segment_repository: Arc::new(segment_repository),
        service_repository: Arc::new(service_repository),
        job_repository: Arc::new(job_repository),
        account_item_repository: Arc::new(account_item_repository),
        jwt_secret: config.jwt_secret,
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let app = Router::new()
        .route("/", get(|| async { "Ghost API v2" }))
        .route("/signup", post(handlers::user::create_user))
        .route("/login", post(handlers::auth::login))
        .route("/users/{uid}", get(handlers::user::get_user))
        .route("/themes", get(handlers::theme::list_themes))
        .route("/themes", post(handlers::theme::create_theme))
        .route("/themes/{tid}", get(handlers::theme::get_theme))
        .route("/themes/{tid}", patch(handlers::theme::update_theme))
        .route("/themes/{tid}", delete(handlers::theme::delete_theme))
        .route("/projects", get(handlers::project::list_projects))
        .route("/projects", post(handlers::project::create_project))
        .route("/projects/{pid}", get(handlers::project::get_project))
        .route("/projects/{pid}", patch(handlers::project::update_project))
        .route("/projects/{pid}", delete(handlers::project::delete_project))
        .route("/segments", get(handlers::segment::list_segment))
        .route("/segments", post(handlers::segment::create_segment))
        .route("/services", get(handlers::service::list_service))
        .route("/services", post(handlers::service::create_service))
        .route(
            "/services/{identifier}",
            get(handlers::service::get_service),
        )
        .route(
            "/services/{identifier}",
            patch(handlers::service::update_service),
        )
        .route(
            "/services/{identifier}",
            delete(handlers::service::delete_service),
        )
        .route("/jobs", get(handlers::job::list_jobs))
        .route("/jobs", post(handlers::job::create_job))
        .route("/jobs/{jid}", get(handlers::job::get_job))
        .route("/jobs/{jid}", patch(handlers::job::update_job))
        .route("/jobs/{jid}", delete(handlers::job::delete_job))
        .route(
            "/account-items",
            get(handlers::account_item::list_account_items),
        )
        .route("/me", get(handlers::auth::get_current_user))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddrV4::new(
        config.host.parse().unwrap_or(Ipv4Addr::new(0, 0, 0, 0)),
        config.port,
    );
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
