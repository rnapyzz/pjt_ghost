use std::sync::Arc;

use crate::domains::user::UserRepository;

pub mod config;
pub mod db;
pub mod domains;
pub mod error;
pub mod handlers;
pub mod repositories;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository>,
    pub jwt_secret: String,
}
