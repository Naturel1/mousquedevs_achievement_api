use chrono::Utc;
use diesel::prelude::*;
use crate::models::profile::{NewProfile, Profile, UpdateProfileRequest, UserProfileView};
use crate::models::user::User;
use crate::models::user_achievement::UserAchievementDetail;
use crate::schema::{achievements, profiles, user_achievements, users};

/// Retrieves the profile associated with a user identifier
pub fn find_by_user_id(conn: &mut PgConnection, target_user_id: i32) -> QueryResult<Profile> {
    profiles::table
        .filter(profiles::user_id.eq(target_user_id))
        .select(Profile::as_select())
        .first(conn)
}

/// Creates a new profile for a user
pub fn create(conn: &mut PgConnection, new_profile: NewProfile) -> QueryResult<Profile> {
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .returning(Profile::as_returning())
        .get_result(conn)
}

/// Updates profile information (bio, avatar_url)
pub fn update(
    conn: &mut PgConnection,
    target_user_id: i32,
    update_data: UpdateProfileRequest,
) -> QueryResult<Profile> {
    let now = Utc::now().naive_utc();
    diesel::update(profiles::table.filter(profiles::user_id.eq(target_user_id)))
        .set((&update_data, profiles::updated_at.eq(now)))
        .returning(Profile::as_returning())
        .get_result(conn)
}

/// Builds the complete user profile view including unlocked achievements
pub fn get_user_profile_view(
    conn: &mut PgConnection,
    target_user_id: i32,
) -> QueryResult<UserProfileView> {
    // 1. Fetch user record
    let user_record: User = users::table
        .find(target_user_id)
        .select(User::as_select())
        .first(conn)?;

    // 2. Fetch profile (or use defaults if not found)
    let profile_opt: Option<Profile> = profiles::table
        .filter(profiles::user_id.eq(target_user_id))
        .select(Profile::as_select())
        .first(conn)
        .optional()?;

    let (bio_str, avatar_str) = match profile_opt {
        Some(p) => (p.bio, p.avatar_url),
        None => ("".to_string(), "".to_string()),
    };

    // 3. Fetch achievements unlocked by the user via inner join
    let obtained_items: Vec<(i32, i32, String, String, i32, chrono::NaiveDateTime)> =
        user_achievements::table
            .inner_join(achievements::table.on(user_achievements::achievement_id.eq(achievements::id)))
            .filter(user_achievements::user_id.eq(target_user_id))
            .select((
                user_achievements::id,
                achievements::id,
                achievements::title,
                achievements::description,
                achievements::points,
                user_achievements::obtained_at,
            ))
            .order(user_achievements::obtained_at.desc())
            .load(conn)?;

    let mut total_points = 0;
    let achievements_list: Vec<UserAchievementDetail> = obtained_items
        .into_iter()
        .map(|(ua_id, ach_id, t, d, pts, obt_at)| {
            total_points += pts;
            UserAchievementDetail {
                user_achievement_id: ua_id,
                achievement_id: ach_id,
                title: t,
                description: d,
                points: pts,
                obtained_at: obt_at,
            }
        })
        .collect();

    let count = achievements_list.len();

    Ok(UserProfileView {
        user_id: user_record.id,
        username: user_record.username,
        role: user_record.role,
        bio: bio_str,
        avatar_url: avatar_str,
        member_since: user_record.created_at,
        total_points,
        achievements_count: count,
        achievements: achievements_list,
    })
}
