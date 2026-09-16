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
            ApiError::NotFound(msg) => {
                log::warn!("404 Not Found on {} {}: {}", req.method(), req.uri(), msg);
                (Status::NotFound, msg)
            }
            ApiError::BadRequest(msg) => {
                log::warn!("400 Bad Request on {} {}: {}", req.method(), req.uri(), msg);
                (Status::BadRequest, msg)
            }
            ApiError::Unauthorized(msg) => {
                log::warn!("401 Unauthorized on {} {}: {}", req.method(), req.uri(), msg);
                (Status::Unauthorized, msg)
            }
            ApiError::Forbidden(msg) => {
                log::warn!("403 Forbidden on {} {}: {}", req.method(), req.uri(), msg);
                (Status::Forbidden, msg)
            }
            ApiError::InternalServerError(msg) => {
                log::error!("500 Internal Server Error on {} {}: {}", req.method(), req.uri(), msg);
                (Status::InternalServerError, msg)
            }
            ApiError::DatabaseError(msg) => {
                log::error!("Database Error on {} {}: {}", req.method(), req.uri(), msg);
                (Status::InternalServerError, msg)
            }
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
