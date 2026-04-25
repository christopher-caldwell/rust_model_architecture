use axum::{
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use auth_core::Claims;
use server_bootstrap::ServerDeps;

fn map_auth_error(error: auth_core::AuthError) -> StatusCode {
    tracing::warn!("JWT error: {error:?}");
    StatusCode::FORBIDDEN
}

fn authenticate_token(deps: &ServerDeps, token: &str) -> Result<Claims, StatusCode> {
    deps.auth
        .verifier
        .verify_token(token)
        .map_err(map_auth_error)
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

    let claims = authenticate_token(&deps, token)?;

    // Stash claims in request extensions
    req.extensions_mut().insert(claims);

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
