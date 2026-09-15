pub mod catchers;

use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Json;
use rocket::Request;
use serde::Serialize;

/// Standard JSON error response format returned to the frontend
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// Application and API error enumeration
#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    InternalServerError(String),
    DatabaseError(String),
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                ApiError::NotFound("Resource not found".to_string())
            }
            other => ApiError::DatabaseError(other.to_string()),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (Status::NotFound, msg),
            ApiError::BadRequest(msg) => (Status::BadRequest, msg),
            ApiError::Unauthorized(msg) => (Status::Unauthorized, msg),
            ApiError::Forbidden(msg) => (Status::Forbidden, msg),
            ApiError::InternalServerError(msg) => (Status::InternalServerError, msg),
            ApiError::DatabaseError(msg) => (Status::InternalServerError, msg),
        };

        let body = Json(ErrorResponse {
            error: status.reason().unwrap_or("Error").to_string(),
            message,
        });

        Response::build_from(body.respond_to(req)?)
            .status(status)
            .ok()
    }
}
