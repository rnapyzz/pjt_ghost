use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{AppState, error::AppError, handlers::auth::Claims};

pub struct AuthUser {
    pub claims: Claims,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::AuthError)?
            .to_str()
            .map_err(|_| AppError::AuthError)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::AuthError);
        }
        let token = &auth_header[7..];

        let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
        let validation = Validation::default();

        let token_data = decode::<Claims>(token, &key, &validation).map_err(|e| {
            tracing::error!("JWT Decode Error: {:?}", e);
            AppError::AuthError
        })?;

        Ok(AuthUser {
            claims: token_data.claims,
        })
    }
}
