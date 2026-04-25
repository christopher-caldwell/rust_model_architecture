use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken(#[source] jsonwebtoken::errors::Error),
}
