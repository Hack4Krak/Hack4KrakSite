use chrono::TimeDelta;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::jwt::{decode_jwt, encode_jwt};
use uuid::Uuid;

#[test]
fn encode_decode_round_trip() {
    EnvConfig::load_test_config();

    let id = Uuid::new_v4();
    let email = "test@example.com".to_string();

    let token = encode_jwt(id, email.clone(), TimeDelta::minutes(10)).unwrap();
    let decoded = decode_jwt(&token).unwrap();

    assert_eq!(decoded.claims.id, id);
    assert_eq!(decoded.claims.email, email);
}

#[test]
fn decode_invalid_token() {
    EnvConfig::load_test_config();

    let result = decode_jwt("not.a.valid.jwt");
    assert!(result.is_err());
}

#[test]
fn decode_empty_token() {
    EnvConfig::load_test_config();

    let result = decode_jwt("");
    assert!(result.is_err());
}

#[test]
fn encode_jwt_preserves_uuid() {
    EnvConfig::load_test_config();

    let id = Uuid::new_v4();
    let token = encode_jwt(id, "a@b.com".to_string(), TimeDelta::hours(1)).unwrap();
    let claims = decode_jwt(&token).unwrap().claims;

    assert_eq!(claims.id, id);
    assert!(claims.expiration_time > claims.issued_at);
}
