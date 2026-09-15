use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::action_logs;

/// Model representing an audit action log in the database
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = action_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ActionLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub action: String,
    pub details: String,
    pub created_at: NaiveDateTime,
}

/// DTO for inserting an action log
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = action_logs)]
pub struct NewActionLog<'a> {
    pub user_id: Option<i32>,
    pub action: &'a str,
    pub details: &'a str,
}
