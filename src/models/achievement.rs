use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::achievements;

/// Model representing an achievement stored in the database
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = achievements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Achievement {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_by_id: Option<i32>,
    pub created_at: NaiveDateTime,
}

/// DTO for creating a new achievement by an administrator (POST)
#[derive(Debug, Clone, Insertable, Deserialize)]
#[diesel(table_name = achievements)]
pub struct NewAchievement {
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_by_id: Option<i32>,
}

/// DTO for updating an existing achievement (PUT)
#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = achievements)]
pub struct UpdateAchievement {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// DTO for proposing a new achievement by a user
#[derive(Debug, Clone, Deserialize)]
pub struct ProposeAchievementRequest {
    pub title: String,
    pub description: String,
}

/// DTO for updating the status of an achievement (admin moderation: "approved", "rejected")
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}
