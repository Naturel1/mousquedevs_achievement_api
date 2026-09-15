use diesel::prelude::*;
use crate::models::user_achievement::{NewUserAchievement, UserAchievement, UserAchievementDetail};
use crate::schema::achievements;
use crate::schema::user_achievements::dsl::*;

/// Retrieves the list of achievements unlocked by a user with achievement details
pub fn find_details_by_user_id(
    conn: &mut PgConnection,
    target_user_id: i32,
) -> QueryResult<Vec<UserAchievementDetail>> {
    let rows: Vec<(i32, i32, String, String, i32, chrono::NaiveDateTime)> = user_achievements
        .inner_join(achievements::table.on(achievement_id.eq(achievements::id)))
        .filter(user_id.eq(target_user_id))
        .select((
            id,
            achievements::id,
            achievements::title,
            achievements::description,
            achievements::points,
            obtained_at,
        ))
        .order(obtained_at.desc())
        .load(conn)?;

    Ok(rows
        .into_iter()
        .map(|(ua_id, ach_id, t, d, pts, obt_at)| UserAchievementDetail {
            user_achievement_id: ua_id,
            achievement_id: ach_id,
            title: t,
            description: d,
            points: pts,
            obtained_at: obt_at,
        })
        .collect())
}

/// Checks if a user already unlocked a specific achievement
pub fn has_achievement(
    conn: &mut PgConnection,
    target_user_id: i32,
    target_achievement_id: i32,
) -> QueryResult<bool> {
    let count: i64 = user_achievements
        .filter(user_id.eq(target_user_id))
        .filter(achievement_id.eq(target_achievement_id))
        .count()
        .get_result(conn)?;

    Ok(count > 0)
}

/// Grants an achievement to a user
pub fn grant(
    conn: &mut PgConnection,
    target_user_id: i32,
    target_achievement_id: i32,
) -> QueryResult<UserAchievement> {
    let new_entry = NewUserAchievement {
        user_id: target_user_id,
        achievement_id: target_achievement_id,
    };

    diesel::insert_into(user_achievements)
        .values(&new_entry)
        .returning(UserAchievement::as_returning())
        .get_result(conn)
}

/// Revokes / removes an achievement from a user
pub fn revoke(
    conn: &mut PgConnection,
    target_user_id: i32,
    target_achievement_id: i32,
) -> QueryResult<usize> {
    diesel::delete(
        user_achievements
            .filter(user_id.eq(target_user_id))
            .filter(achievement_id.eq(target_achievement_id)),
    )
    .execute(conn)
}
