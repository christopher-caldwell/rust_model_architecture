use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::{AuthError, AuthVerifierPort, Claims};

pub const JWT_AUDIENCE: &str = "ops.craftcode.solutions";

#[derive(Clone)]
pub struct JwtAuthAdapter {
    jwt_secret: String,
}

impl JwtAuthAdapter {
    #[must_use]
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl AuthVerifierPort for JwtAuthAdapter {
    fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[JWT_AUDIENCE]);

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .map(|token_data| token_data.claims)
        .map_err(AuthError::InvalidToken)
    }
}
