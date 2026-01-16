use std::sync::Arc;

use crate::domains::{service::ServiceRepository, theme::ThemeRepository, user::UserRepository};

pub mod config;
pub mod db;
pub mod domains;
pub mod error;
pub mod extractors;
pub mod handlers;
pub mod repositories;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub theme_repository: Arc<dyn ThemeRepository>,
    pub service_repository: Arc<dyn ServiceRepository>,
    pub jwt_secret: String,
}
