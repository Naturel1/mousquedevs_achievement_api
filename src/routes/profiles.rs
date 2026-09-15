use rocket::serde::json::Json;
use rocket::{get, put, routes, Route};
use crate::auth::jwt::AuthenticatedUser;
use crate::db::DbConn;
use crate::errors::ApiError;
use crate::models::profile::{Profile, UpdateProfileRequest, UserProfileView};
use crate::repositories::{log_repository, profile_repository};

/// GET /api/profiles/<user_id>
/// Retrieves the public profile of a user by ID along with their unlocked achievements
#[get("/<user_id>")]
pub async fn get_profile_by_id(
    db: DbConn,
    user_id: i32,
) -> Result<Json<UserProfileView>, ApiError> {
    db.run(move |conn| profile_repository::get_user_profile_view(conn, user_id))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// PUT /api/profiles/me
/// Updates the profile of the currently authenticated user (bio, avatar_url)
#[put("/me", data = "<update_data>")]
pub async fn update_my_profile(
    db: DbConn,
    auth: AuthenticatedUser,
    update_data: Json<UpdateProfileRequest>,
) -> Result<Json<Profile>, ApiError> {
    let data = update_data.into_inner();
    db.run(move |conn| {
        let updated = profile_repository::update(conn, auth.id, data)?;
        let _ = log_repository::log_action(
            conn,
            Some(auth.id),
            "PROFILE_UPDATE",
            &format!("Profile update for '{}'", auth.username),
        );
        Ok(Json(updated))
    })
    .await
}

/// Returns all routes for profile management
pub fn routes_profiles() -> Vec<Route> {
    routes![get_profile_by_id, update_my_profile]
}
