use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes, Route};
use crate::auth::jwt::{AdminUser, AuthenticatedUser};
use crate::db::DbConn;
use crate::errors::ApiError;
use crate::models::achievement::{
    Achievement, NewAchievement, ProposeAchievementRequest, UpdateAchievement,
};
use crate::models::user_achievement::UserAchievement;
use crate::repositories::{
    achievement_repository, log_repository, user_achievement_repository,
};

/// GET /api/achievements
/// Retrieves the list of all validated / public achievements
#[get("/")]
pub async fn get_all(db: DbConn) -> Result<Json<Vec<Achievement>>, ApiError> {
    db.run(|conn| achievement_repository::find_all_approved(conn))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// GET /api/achievements/<id>
/// Retrieves an achievement by its identifier
#[get("/<id>")]
pub async fn get_by_id(db: DbConn, id: i32) -> Result<Json<Achievement>, ApiError> {
    db.run(move |conn| achievement_repository::find_by_id(conn, id))
        .await
        .map(Json)
        .map_err(ApiError::from)
}

/// POST /api/achievements/propose
/// Allows an authenticated user to propose a new achievement to the administration
#[post("/propose", data = "<proposal>")]
pub async fn propose_achievement(
    db: DbConn,
    auth: AuthenticatedUser,
    proposal: Json<ProposeAchievementRequest>,
) -> Result<Custom<Json<Achievement>>, ApiError> {
    let req = proposal.into_inner();
    let title_trimmed = req.title.trim().to_string();
    let desc_trimmed = req.description.trim().to_string();

    if title_trimmed.is_empty() || desc_trimmed.is_empty() {
        return Err(ApiError::BadRequest(
            "Achievement title and description are required".to_string(),
        ));
    }

    let points_val = req.points.unwrap_or(10);
    if points_val < 0 {
        return Err(ApiError::BadRequest(
            "Achievement points must be positive".to_string(),
        ));
    }

    db.run(move |conn| {
        let new_item = NewAchievement {
            title: title_trimmed,
            description: desc_trimmed,
            points: points_val,
            status: "pending".to_string(),
            created_by_id: Some(auth.id),
        };

        let created = achievement_repository::create(conn, new_item)?;

        let _ = log_repository::log_action(
            conn,
            Some(auth.id),
            "ACHIEVEMENT_PROPOSED",
            &format!(
                "User '{}' proposed achievement '{}' (id: {})",
                auth.username, created.title, created.id
            ),
        );

        Ok(Custom(Status::Created, Json(created)))
    })
    .await
}

/// POST /api/achievements/obtain/<id>
/// Allows the authenticated user to unlock / obtain an approved achievement
#[post("/obtain/<id>")]
pub async fn obtain_achievement(
    db: DbConn,
    auth: AuthenticatedUser,
    id: i32,
) -> Result<Custom<Json<UserAchievement>>, ApiError> {
    db.run(move |conn| {
        // 1. Verify that the achievement exists and is approved
        let achievement = achievement_repository::find_by_id(conn, id)?;
        if achievement.status != "approved" {
            return Err(ApiError::BadRequest(
                "This achievement is not available for unlocking".to_string(),
            ));
        }

        // 2. Verify that the user has not already unlocked it
        let already_has = user_achievement_repository::has_achievement(conn, auth.id, id)?;
        if already_has {
            return Err(ApiError::BadRequest(
                "You have already unlocked this achievement".to_string(),
            ));
        }

        // 3. Grant the achievement with the current date (obtained_at)
        let record = user_achievement_repository::grant(conn, auth.id, id)?;

        // 4. Log the achievement unlocking
        let _ = log_repository::log_action(
            conn,
            Some(auth.id),
            "ACHIEVEMENT_OBTAINED",
            &format!(
                "User '{}' unlocked achievement '{}' (+{} points)",
                auth.username, achievement.title, achievement.points
            ),
        );

        Ok(Custom(Status::Created, Json(record)))
    })
    .await
}

/// POST /api/achievements
/// Directly creates a new approved achievement (Admin only)
#[post("/", data = "<new_item>")]
pub async fn create(
    db: DbConn,
    admin: AdminUser,
    new_item: Json<ProposeAchievementRequest>,
) -> Result<Custom<Json<Achievement>>, ApiError> {
    let req = new_item.into_inner();
    let title_trimmed = req.title.trim().to_string();
    let desc_trimmed = req.description.trim().to_string();
    let points_val = req.points.unwrap_or(10);

    if title_trimmed.is_empty() || desc_trimmed.is_empty() {
        return Err(ApiError::BadRequest(
            "Title and description are required".to_string(),
        ));
    }

    db.run(move |conn| {
        let item = NewAchievement {
            title: title_trimmed,
            description: desc_trimmed,
            points: points_val,
            status: "approved".to_string(),
            created_by_id: Some(admin.id),
        };

        let created = achievement_repository::create(conn, item)?;

        let _ = log_repository::log_action(
            conn,
            Some(admin.id),
            "ACHIEVEMENT_CREATED",
            &format!(
                "Administrator '{}' created achievement '{}' (id: {})",
                admin.username, created.title, created.id
            ),
        );

        Ok(Custom(Status::Created, Json(created)))
    })
    .await
}

/// PUT /api/achievements/<id>
/// Updates an existing achievement (Admin only)
#[put("/<id>", data = "<update_data>")]
pub async fn update(
    db: DbConn,
    admin: AdminUser,
    id: i32,
    update_data: Json<UpdateAchievement>,
) -> Result<Json<Achievement>, ApiError> {
    let data = update_data.into_inner();
    db.run(move |conn| {
        let updated = achievement_repository::update(conn, id, data)?;

        let _ = log_repository::log_action(
            conn,
            Some(admin.id),
            "ACHIEVEMENT_UPDATED",
            &format!(
                "Administrator '{}' updated achievement (id: {})",
                admin.username, id
            ),
        );

        Ok(Json(updated))
    })
    .await
}

/// DELETE /api/achievements/<id>
/// Deletes an existing achievement (Admin only)
#[delete("/<id>")]
pub async fn delete(
    db: DbConn,
    admin: AdminUser,
    id: i32,
) -> Result<Custom<Json<serde_json::Value>>, ApiError> {
    db.run(move |conn| {
        let count = achievement_repository::delete(conn, id)?;
        if count == 0 {
            Err(ApiError::NotFound(format!(
                "Achievement with identifier {} does not exist",
                id
            )))
        } else {
            let _ = log_repository::log_action(
                conn,
                Some(admin.id),
                "ACHIEVEMENT_DELETED",
                &format!(
                    "Administrator '{}' deleted achievement (id: {})",
                    admin.username, id
                ),
            );

            Ok(Custom(
                Status::Ok,
                Json(serde_json::json!({
                    "message": "Achievement successfully deleted",
                    "id": id
                })),
            ))
        }
    })
    .await
}

/// Returns all routes for the achievements module
pub fn routes() -> Vec<Route> {
    routes![
        get_all,
        get_by_id,
        propose_achievement,
        obtain_achievement,
        create,
        update,
        delete
    ]
}
