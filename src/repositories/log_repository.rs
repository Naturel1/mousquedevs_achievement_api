use diesel::prelude::*;
use crate::models::action_log::{ActionLog, NewActionLog};
use crate::schema::action_logs::dsl::*;

/// Records an action in the audit log
pub fn log_action(
    conn: &mut PgConnection,
    uid: Option<i32>,
    act: &str,
    det: &str,
) -> QueryResult<ActionLog> {
    let new_log = NewActionLog {
        user_id: uid,
        action: act,
        details: det,
    };

    diesel::insert_into(action_logs)
        .values(&new_log)
        .returning(ActionLog::as_returning())
        .get_result(conn)
}

/// Retrieves the list of the most recent audit logs
pub fn find_all(conn: &mut PgConnection, limit_count: i64) -> QueryResult<Vec<ActionLog>> {
    action_logs
        .order(created_at.desc())
        .limit(limit_count)
        .select(ActionLog::as_select())
        .load(conn)
}

/// Retrieves the audit logs of a specific user
pub fn find_by_user_id(
    conn: &mut PgConnection,
    target_user_id: i32,
    limit_count: i64,
) -> QueryResult<Vec<ActionLog>> {
    action_logs
        .filter(user_id.eq(target_user_id))
        .order(created_at.desc())
        .limit(limit_count)
        .select(ActionLog::as_select())
        .load(conn)
}
