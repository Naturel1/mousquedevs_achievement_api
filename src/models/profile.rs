use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::user_achievement::UserAchievementDetail;
use crate::schema::profiles;

/// Model representing a user profile
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub bio: String,
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// DTO for creating a profile
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = profiles)]
pub struct NewProfile<'a> {
    pub user_id: i32,
    pub bio: &'a str,
    pub avatar_url: &'a str,
}

/// DTO for updating an existing profile
#[derive(Debug, Clone, AsChangeset, Deserialize)]
#[diesel(table_name = profiles)]
pub struct UpdateProfileRequest {
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// Complete view of a user profile with unlocked achievements
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileView {
    pub user_id: i32,
    pub username: String,
    pub role: String,
    pub bio: String,
    pub avatar_url: String,
    pub member_since: NaiveDateTime,
    pub total_points: i32,
    pub achievements_count: usize,
    pub achievements: Vec<UserAchievementDetail>,
}
