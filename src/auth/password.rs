use bcrypt::{hash, verify, DEFAULT_COST};
use crate::errors::ApiError;

/// Hashes a plain-text password using the Bcrypt algorithm
pub fn hash_password(password: &str) -> Result<String, ApiError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to hash password: {}", e)))
}

/// Verifies that a plain-text password matches a Bcrypt hash
pub fn verify_password(password: &str, hash_str: &str) -> Result<bool, ApiError> {
    verify(password, hash_str)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to verify password: {}", e)))
}
