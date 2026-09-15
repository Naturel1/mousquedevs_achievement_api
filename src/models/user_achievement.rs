use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::user_achievements;

/// Model representing the unlocking of an achievement by a user
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = user_achievements)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAchievement {
    pub id: i32,
    pub user_id: i32,
    pub achievement_id: i32,
    pub obtained_at: NaiveDateTime,
}

/// DTO for recording the unlocking of an achievement
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = user_achievements)]
pub struct NewUserAchievement {
    pub user_id: i32,
    pub achievement_id: i32,
}

/// Detailed DTO of an unlocked achievement (including achievement details)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAchievementDetail {
    pub user_achievement_id: i32,
    pub achievement_id: i32,
    pub title: String,
    pub description: String,
    pub points: i32,
    pub obtained_at: NaiveDateTime,
}
