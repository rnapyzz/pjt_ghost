use axum::http::StatusCode;
use uuid::Uuid;

use crate::domain::users::Claims;

/// Claimsからuser_idを安全に取り出し、Uuidに変換するヘルパー関数
pub fn extract_user_id(claims: &Claims) -> Result<Uuid, (StatusCode, String)> {
    Uuid::parse_str(&claims.sub).map_err(|e| {
        tracing::error!("Invalid UUID format in token: {:?}", e);
        (StatusCode::UNAUTHORIZED, e.to_string())
    })
}
