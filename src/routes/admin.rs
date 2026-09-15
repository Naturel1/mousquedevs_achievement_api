use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use crate::auth::jwt::AdminUser;
use crate::db::DbConn;
use crate::errors::ApiError;
use crate::models::achievement::{Achievement, UpdateStatusRequest};
use crate::models::action_log::ActionLog;
use crate::models::user::{UpdateUserRoleRequest, UserResponse};
use crate::models::user_achievement::UserAchievement;
use crate::repositories::{
    achievement_repository, log_repository, user_achievement_repository, user_repository,
};

/// GET /api/admin/users
/// Retrieves the list of all registered users
#[get("/users")]
pub async fn list_users(
    db: DbConn,
    _admin: AdminUser,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    db.run(|conn| {
        let users = user_repository::find_all(conn)?;
        let responses = users.into_iter().map(UserResponse::from).collect();
        Ok(Json(responses))
    })
    .await
}

/// PUT /api/admin/users/<id>/role
/// Updates a user's role (e.g. 'user' or 'admin')
#[put("/users/<id>/role", data = "<req_data>")]
pub async fn update_user_role(
    db: DbConn,
    admin: AdminUser,
    id: i32,
    req_data: Json<UpdateUserRoleRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let new_role = req_data.into_inner().role.trim().to_lowercase();
    if new_role != "user" && new_role != "admin" {
        return Err(ApiError::BadRequest(
            "Role must be either 'user' or 'admin'".to_string(),
        ));
    }

    db.run(move |conn| {
        let updated = user_repository::update_role(conn, id, &new_role)?;
        let _ = log_repository::log_action(
            conn,
            Some(admin.id),
            "USER_ROLE_UPDATED",
            &format!(
                "Administrator '{}' changed the role of user '{}' (id: {}) to '{}'",
                admin.username, updated.username, id, new_role
            ),
        );
        Ok(Json(UserResponse::from(updated)))
    })
    .await
}

/// GET /api/admin/achievements
/// Retrieves all achievements (approved, pending, rejected)
#[get("/achievements")]
pub async fn list_all_achievements(
    db: DbConn,
    _admin: AdminUser,
) -> Result<Json<Vec<Achievement>>, ApiError> {
    db.run(|conn| achievement_repository::find_all(conn))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// GET /api/admin/achievements/pending
/// Retrieves pending achievement proposals awaiting moderation
#[get("/achievements/pending")]
pub async fn list_pending_achievements(
    db: DbConn,
    _admin: AdminUser,
) -> Result<Json<Vec<Achievement>>, ApiError> {
    db.run(|conn| achievement_repository::find_by_status(conn, "pending"))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// PUT /api/admin/achievements/<id>/status
/// Approves or rejects an achievement proposal ('approved', 'rejected')
#[put("/achievements/<id>/status", data = "<req_data>")]
pub async fn update_achievement_status(
    db: DbConn,
    admin: AdminUser,
    id: i32,
    req_data: Json<UpdateStatusRequest>,
) -> Result<Json<Achievement>, ApiError> {
    let new_status = req_data.into_inner().status.trim().to_lowercase();
    if new_status != "approved" && new_status != "rejected" && new_status != "pending" {
        return Err(ApiError::BadRequest(
            "Status must be 'approved', 'rejected' or 'pending'".to_string(),
        ));
    }

    db.run(move |conn| {
        let updated = achievement_repository::update_status(conn, id, &new_status)?;
        let _ = log_repository::log_action(
            conn,
            Some(admin.id),
            "ACHIEVEMENT_STATUS_UPDATED",
            &format!(
                "Administrator '{}' changed the status of achievement '{}' (id: {}) to '{}'",
                admin.username, updated.title, id, new_status
            ),
        );
        Ok(Json(updated))
    })
    .await
}

/// POST /api/admin/users/<user_id>/grant/<achievement_id>
/// Manually grants an achievement to a user from the administration panel
#[post("/users/<user_id>/grant/<achievement_id>")]
pub async fn grant_user_achievement(
    db: DbConn,
    admin: AdminUser,
    user_id: i32,
    achievement_id: i32,
) -> Result<Custom<Json<UserAchievement>>, ApiError> {
    db.run(move |conn| {
        let user = user_repository::find_by_id(conn, user_id)?;
        let achievement = achievement_repository::find_by_id(conn, achievement_id)?;

        let already_has =
            user_achievement_repository::has_achievement(conn, user_id, achievement_id)?;
        if already_has {
            return Err(ApiError::BadRequest(
                "User already has this achievement".to_string(),
            ));
        }

        let record = user_achievement_repository::grant(conn, user_id, achievement_id)?;
        let _ = log_repository::log_action(
            conn,
            Some(admin.id),
            "ADMIN_GRANT_ACHIEVEMENT",
            &format!(
                "Administrator '{}' granted achievement '{}' (id: {}) to '{}' (id: {})",
                admin.username, achievement.title, achievement_id, user.username, user_id
            ),
        );

        Ok(Custom(Status::Created, Json(record)))
    })
    .await
}

/// DELETE /api/admin/users/<user_id>/revoke/<achievement_id>
/// Revokes / removes an achievement from a user
#[delete("/users/<user_id>/revoke/<achievement_id>")]
pub async fn revoke_user_achievement(
    db: DbConn,
    admin: AdminUser,
    user_id: i32,
    achievement_id: i32,
) -> Result<Custom<Json<serde_json::Value>>, ApiError> {
    db.run(move |conn| {
        let user = user_repository::find_by_id(conn, user_id)?;
        let count = user_achievement_repository::revoke(conn, user_id, achievement_id)?;
        if count == 0 {
            return Err(ApiError::NotFound(
                "This achievement was not granted to this user".to_string(),
            ));
        }

        let _ = log_repository::log_action(
            conn,
            Some(admin.id),
            "ADMIN_REVOKE_ACHIEVEMENT",
            &format!(
                "Administrator '{}' revoked achievement id: {} from '{}' (id: {})",
                admin.username, achievement_id, user.username, user_id
            ),
        );

        Ok(Custom(
            Status::Ok,
            Json(serde_json::json!({
                "message": "Achievement successfully revoked",
                "user_id": user_id,
                "achievement_id": achievement_id
            })),
        ))
    })
    .await
}

/// GET /api/admin/logs
/// Retrieves the system action audit history
#[get("/logs?<limit>")]
pub async fn get_logs(
    db: DbConn,
    _admin: AdminUser,
    limit: Option<i64>,
) -> Result<Json<Vec<ActionLog>>, ApiError> {
    let limit_val = limit.unwrap_or(100).clamp(1, 500);
    db.run(move |conn| log_repository::find_all(conn, limit_val))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// Returns all routes of the administration panel
pub fn routes() -> Vec<Route> {
    routes![
        list_users,
        update_user_role,
        list_all_achievements,
        list_pending_achievements,
        update_achievement_status,
        grant_user_achievement,
        revoke_user_achievement,
        get_logs
    ]
}
