use rusty_saas::{AuthService, Config};
use std::sync::Arc;

#[tokio::test]
async fn test_password_hashing() {
    let config = Arc::new(Config::default());
    let auth_service = AuthService::new(Arc::new(config.jwt.clone()));

    let password = "test_password_123";
    let hash = auth_service.hash_password(password).unwrap();

    // Verify that the password matches the hash
    assert!(auth_service.verify_password(password, &hash).unwrap());

    // Verify that a different password does not match
    assert!(!auth_service
        .verify_password("wrong_password", &hash)
        .unwrap());
}

#[tokio::test]
async fn test_jwt_token_generation_and_validation() {
    let config = Arc::new(Config::default());
    let auth_service = AuthService::new(Arc::new(config.jwt.clone()));

    let user_id = "test-user-id";
    let email = "test@example.com";

    // Generate token
    let token = auth_service.generate_token(user_id, email).unwrap();

    // Validate token
    let claims = auth_service.validate_token(&token).unwrap();

    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.email, email);
}

#[test]
fn test_extract_token_from_header() {
    let valid_header = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
    let result = AuthService::extract_token_from_header(valid_header);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9");

    let invalid_header = "Basic some-credentials";
    let result = AuthService::extract_token_from_header(invalid_header);
    assert!(result.is_err());
}

#[test]
fn test_config_default() {
    let config = Config::default();

    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 8080);
    assert_eq!(config.database.max_connections, 10);
}
