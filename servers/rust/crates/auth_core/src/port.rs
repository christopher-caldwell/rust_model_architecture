use crate::{AuthError, Claims};

pub trait AuthVerifierPort {
    fn verify_token(&self, token: &str) -> Result<Claims, AuthError>;
}
