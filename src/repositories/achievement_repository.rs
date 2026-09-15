use diesel::prelude::*;
use crate::models::achievement::{Achievement, NewAchievement, UpdateAchievement};
use crate::schema::achievements;
use crate::schema::achievements::dsl::*;

/// Retrieves all validated / approved achievements (visible to users)
pub fn find_all_approved(conn: &mut PgConnection) -> QueryResult<Vec<Achievement>> {
    achievements
        .filter(status.eq("approved"))
        .order(id.asc())
        .select(Achievement::as_select())
        .load(conn)
}

/// Retrieves all achievements regardless of their status (for the admin panel)
pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<Achievement>> {
    achievements
        .order(id.asc())
        .select(Achievement::as_select())
        .load(conn)
}

/// Retrieves achievements filtered by status (e.g., 'pending' for proposals)
pub fn find_by_status(conn: &mut PgConnection, target_status: &str) -> QueryResult<Vec<Achievement>> {
    achievements
        .filter(status.eq(target_status))
        .order(created_at.desc())
        .select(Achievement::as_select())
        .load(conn)
}

/// Retrieves a specific achievement by its identifier
pub fn find_by_id(conn: &mut PgConnection, achievement_id: i32) -> QueryResult<Achievement> {
    achievements
        .find(achievement_id)
        .select(Achievement::as_select())
        .first(conn)
}

/// Inserts a new achievement into the database
pub fn create(conn: &mut PgConnection, new_item: NewAchievement) -> QueryResult<Achievement> {
    diesel::insert_into(achievements::table)
        .values(&new_item)
        .returning(Achievement::as_returning())
        .get_result(conn)
}

/// Updates fields of an existing achievement
pub fn update(
    conn: &mut PgConnection,
    achievement_id: i32,
    update_data: UpdateAchievement,
) -> QueryResult<Achievement> {
    diesel::update(achievements.find(achievement_id))
        .set(&update_data)
        .returning(Achievement::as_returning())
        .get_result(conn)
}

/// Updates the status of an achievement (e.g., 'approved', 'rejected')
pub fn update_status(
    conn: &mut PgConnection,
    achievement_id: i32,
    new_status: &str,
) -> QueryResult<Achievement> {
    diesel::update(achievements.find(achievement_id))
        .set(status.eq(new_status))
        .returning(Achievement::as_returning())
        .get_result(conn)
}

/// Deletes an achievement by its identifier
pub fn delete(conn: &mut PgConnection, achievement_id: i32) -> QueryResult<usize> {
    diesel::delete(achievements.find(achievement_id)).execute(conn)
}
