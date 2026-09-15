use rocket::serde::json::Json;
use rocket::{catch, catchers, Catcher};
use super::ErrorResponse;

#[catch(400)]
pub fn bad_request() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Bad Request".to_string(),
        message: "The request sent is invalid".to_string(),
    })
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Unauthorized".to_string(),
        message: "Authentication required or invalid token".to_string(),
    })
}

#[catch(403)]
pub fn forbidden() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Forbidden".to_string(),
        message: "You do not have the required permissions to access this resource".to_string(),
    })
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Not Found".to_string(),
        message: "The requested route or resource does not exist".to_string(),
    })
}

#[catch(422)]
pub fn unprocessable_entity() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Unprocessable Entity".to_string(),
        message: "The request body is malformed or does not match the expected format".to_string(),
    })
}

#[catch(500)]
pub fn internal_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Internal Server Error".to_string(),
        message: "An internal error occurred on the server".to_string(),
    })
}

/// Returns all configured catchers for the application
pub fn all_catchers() -> Vec<Catcher> {
    catchers![
        bad_request,
        unauthorized,
        forbidden,
        not_found,
        unprocessable_entity,
        internal_error
    ]
}
