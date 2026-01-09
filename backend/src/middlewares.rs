use axum::{
    extract::{Request, State},
    http::{StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Validation, decode};

use crate::{AppState, domain::users::Claims};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = if let Some(token) = auth_header.strip_prefix("Bearer ") {
        token
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = decode::<Claims>(token, &state.jwt_decoding_key, &Validation::default())
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
