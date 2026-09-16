use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use crate::auth::jwt::{create_jwt, AuthenticatedUser};
use crate::auth::password::{hash_password, verify_password};
use crate::db::DbConn;
use crate::errors::ApiError;
use crate::models::profile::{NewProfile, UserProfileView};
use crate::models::user::{AuthResponse, LoginRequest, NewUser, RegisterRequest, UserResponse};
use crate::repositories::{profile_repository, user_repository};

/// POST /api/auth/register
/// Registers a new user account
#[post("/register", data = "<req_data>")]
pub async fn register(
    db: DbConn,
    req_data: Json<RegisterRequest>,
) -> Result<Custom<Json<AuthResponse>>, ApiError> {
    let req = req_data.into_inner();

    // Input validation
    let username_trimmed = req.username.trim();
    let email_trimmed = req.email.trim();
    let password_str = req.password.trim();

    if username_trimmed.len() < 3 || username_trimmed.len() > 50 {
        return Err(ApiError::BadRequest(
            "Username must be between 3 and 50 characters".to_string(),
        ));
    }

    if !email_trimmed.contains('@') || email_trimmed.len() < 5 {
        return Err(ApiError::BadRequest(
            "The provided email address is invalid".to_string(),
        ));
    }

    if password_str.len() < 6 {
        return Err(ApiError::BadRequest(
            "Password must contain at least 6 characters".to_string(),
        ));
    }

    let hashed = hash_password(password_str)?;
    let username_owned = username_trimmed.to_string();
    let email_owned = email_trimmed.to_string();

    db.run(move |conn| {
        // Uniqueness validation
        if user_repository::find_by_username(conn, &username_owned).is_ok() {
            log::warn!("Registration failed: username '{}' is already taken", username_owned);
            return Err(ApiError::BadRequest(
                "This username is already taken".to_string(),
            ));
        }

        if user_repository::find_by_email(conn, &email_owned).is_ok() {
            log::warn!("Registration failed: email '{}' is already in use", email_owned);
            return Err(ApiError::BadRequest(
                "This email address is already in use".to_string(),
            ));
        }

        // If this is the very first user registered in the database, assign the 'admin' role
        let total_users = user_repository::count(conn).unwrap_or(0);
        let assigned_role = if total_users == 0 { "admin" } else { "user" };

        let new_user = NewUser {
            username: &username_owned,
            email: &email_owned,
            password_hash: &hashed,
            role: assigned_role,
        };

        let user = user_repository::create(conn, new_user)?;

        // Create the associated initial profile
        let new_profile = NewProfile {
            user_id: user.id,
            bio: "",
            avatar_url: "",
        };
        profile_repository::create(conn, new_profile)?;

        log::info!(
            "New account registered: '{}' (id: {}) with role '{}'",
            user.username, user.id, user.role
        );

        // Generate JWT token
        let token = create_jwt(user.id, &user.username, &user.role)?;

        Ok(Custom(
            Status::Created,
            Json(AuthResponse {
                token,
                user: UserResponse::from(user),
            }),
        ))
    })
    .await
}

/// POST /api/auth/login
/// Authenticates an existing user
#[post("/login", data = "<req_data>")]
pub async fn login(
    db: DbConn,
    req_data: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let req = req_data.into_inner();
    let identifier = req.username_or_email.trim().to_string();
    let password_str = req.password.trim().to_string();

    if identifier.is_empty() || password_str.is_empty() {
        return Err(ApiError::BadRequest(
            "Identifier and password are required".to_string(),
        ));
    }

    db.run(move |conn| {
        let user = user_repository::find_by_username_or_email(conn, &identifier)
            .map_err(|_| {
                log::warn!("Authentication failed: user identifier '{}' not found", identifier);
                ApiError::Unauthorized("Invalid credentials".to_string())
            })?;

        let is_valid = verify_password(&password_str, &user.password_hash)?;
        if !is_valid {
            log::warn!("Authentication failed: invalid password for user '{}'", user.username);
            return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
        }

        log::info!("User '{}' (id: {}) successfully logged in", user.username, user.id);

        let token = create_jwt(user.id, &user.username, &user.role)?;

        Ok(Json(AuthResponse {
            token,
            user: UserResponse::from(user),
        }))
    })
    .await
}

/// GET /api/auth/me
/// Retrieves full profile and achievement details for the currently authenticated user
#[get("/me")]
pub async fn get_current_user(
    db: DbConn,
    auth: AuthenticatedUser,
) -> Result<Json<UserProfileView>, ApiError> {
    db.run(move |conn| profile_repository::get_user_profile_view(conn, auth.id))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// Returns all routes for the authentication module
pub fn routes_auth() -> Vec<Route> {
    routes![register, login, get_current_user]
}
