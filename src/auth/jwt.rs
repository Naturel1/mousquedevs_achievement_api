use std::env;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use crate::errors::ApiError;

/// Default JWT secret key for local development
const DEFAULT_JWT_SECRET: &str = "mousquedevs_super_secret_jwt_key_change_in_production";

/// JWT token expiration duration (in hours)
const JWT_EXPIRATION_HOURS: i64 = 24;

/// Data structure (Claims) stored in the JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User identifier (subject)
    pub sub: i32,
    /// Username
    pub username: String,
    /// User role ("user" or "admin")
    pub role: String,
    /// Expiration timestamp (in Unix seconds)
    pub exp: usize,
}

/// Retrieves the configured JWT secret key from the environment
fn get_jwt_secret() -> Vec<u8> {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| DEFAULT_JWT_SECRET.to_string())
        .into_bytes()
}

/// Generates a new JWT token for a given user
pub fn create_jwt(user_id: i32, username: &str, role: &str) -> Result<String, ApiError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(JWT_EXPIRATION_HOURS))
        .expect("Failed to calculate token expiration date")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&get_jwt_secret()),
    )
    .map_err(|e| ApiError::InternalServerError(format!("Failed to generate JWT token: {}", e)))
}

/// Decodes and validates a JWT token
pub fn decode_jwt(token: &str) -> Result<Claims, ApiError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&get_jwt_secret()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::Unauthorized("Invalid or expired JWT token".to_string()))
}

/// Rocket Request Guard: Authenticated user (any role)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub role: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");
        match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..];
                match decode_jwt(token) {
                    Ok(claims) => Outcome::Success(AuthenticatedUser {
                        id: claims.sub,
                        username: claims.username,
                        role: claims.role,
                    }),
                    Err(err) => Outcome::Error((Status::Unauthorized, err)),
                }
            }
            _ => Outcome::Error((
                Status::Unauthorized,
                ApiError::Unauthorized(
                    "Missing or invalid Authorization header (expected format: Bearer <token>)".to_string(),
                ),
            )),
        }
    }
}

/// Rocket Request Guard: Administrator only (role == "admin")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUser {
    pub id: i32,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ApiError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match AuthenticatedUser::from_request(req).await {
            Outcome::Success(user) => {
                if user.role == "admin" {
                    Outcome::Success(AdminUser {
                        id: user.id,
                        username: user.username,
                    })
                } else {
                    Outcome::Error((
                        Status::Forbidden,
                        ApiError::Forbidden("Access restricted to administrators".to_string()),
                    ))
                }
            }
            Outcome::Error((status, err)) => Outcome::Error((status, err)),
            Outcome::Forward(forward) => Outcome::Forward(forward),
        }
    }
}
