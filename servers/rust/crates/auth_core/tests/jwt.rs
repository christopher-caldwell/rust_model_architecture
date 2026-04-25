use auth_core::{AuthVerifierPort, Claims, JwtAuthAdapter};
use jsonwebtoken::{encode, EncodingKey, Header};

fn sample_claims() -> Claims {
    Claims {
        sub: "member-123".to_string(),
        exp: 2_000_000_000,
    }
}

#[test]
fn jwt_adapter_returns_the_original_claims() {
    let secret = "test-secret";
    let token = encode(
        &Header::default(),
        &sample_claims(),
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("token");

    let verifier = JwtAuthAdapter::new(secret.to_string());
    let claims = verifier.verify_token(&token).expect("claims");

    assert_eq!(claims.sub, "member-123");
    assert_eq!(claims.exp, 2_000_000_000);
}

#[test]
fn jwt_adapter_rejects_the_wrong_secret() {
    let token = encode(
        &Header::default(),
        &sample_claims(),
        &EncodingKey::from_secret(b"test-secret"),
    )
    .expect("token");

    let verifier = JwtAuthAdapter::new("different-secret".to_string());
    let error = verifier.verify_token(&token).expect_err("error");

    assert!(matches!(
        error,
        auth_core::AuthError::InvalidToken(inner)
            if matches!(inner.kind(), jsonwebtoken::errors::ErrorKind::InvalidSignature)
    ));
}
