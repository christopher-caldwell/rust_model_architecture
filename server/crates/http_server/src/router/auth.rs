use axum::{
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use server_bootstrap::ServerDeps;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    // add whatever fields your JWT contains
}

pub async fn auth_middleware(
    State(deps): State<ServerDeps>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["ops.craftcode.solutions"]);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(deps.auth.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|e| {
        tracing::warn!("JWT error: {e:?}");
        StatusCode::FORBIDDEN
    })?;

    // Stash claims in request extensions
    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}

pub struct AuthUser(pub Claims);

impl FromRequestParts<ServerDeps> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        _deps: &ServerDeps,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthUser)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}
