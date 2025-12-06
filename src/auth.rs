use crate::config::JwtConfig;
use crate::error::{AppError, Result};
use crate::models::Claims;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::sync::Arc;

/// Authentication service
#[derive(Clone)]
pub struct AuthService {
    config: Arc<JwtConfig>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(config: Arc<JwtConfig>) -> Self {
        Self { config }
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalServerError(format!("Invalid hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user_id: &str, email: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(self.config.expiration_hours))
            .ok_or_else(|| {
                AppError::InternalServerError("Failed to calculate expiration".to_string())
            })?
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_bytes()),
        )
        .map_err(AppError::JwtError)
    }

    /// Validate a JWT token and extract claims
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(AppError::JwtError)?;

        Ok(token_data.claims)
    }

    /// Extract token from Authorization header
    pub fn extract_token_from_header(auth_header: &str) -> Result<&str> {
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Authentication(
                "Invalid authorization header format".to_string(),
            ));
        }

        Ok(&auth_header[7..])
    }
}
