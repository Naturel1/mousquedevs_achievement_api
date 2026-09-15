use diesel::prelude::*;
use crate::models::user::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;

/// Retrieves a user by their unique identifier
pub fn find_by_id(conn: &mut PgConnection, user_id_val: i32) -> QueryResult<User> {
    users
        .find(user_id_val)
        .select(User::as_select())
        .first(conn)
}

/// Retrieves a user by their username
pub fn find_by_username(conn: &mut PgConnection, username_val: &str) -> QueryResult<User> {
    users
        .filter(username.eq(username_val))
        .select(User::as_select())
        .first(conn)
}

/// Retrieves a user by their email address
pub fn find_by_email(conn: &mut PgConnection, email_val: &str) -> QueryResult<User> {
    users
        .filter(email.eq(email_val))
        .select(User::as_select())
        .first(conn)
}

/// Finds a user either by their username or email address
pub fn find_by_username_or_email(conn: &mut PgConnection, identifier: &str) -> QueryResult<User> {
    users
        .filter(username.eq(identifier).or(email.eq(identifier)))
        .select(User::as_select())
        .first(conn)
}

/// Creates a new user
pub fn create(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

/// Updates a user's role (e.g. 'user' -> 'admin')
pub fn update_role(conn: &mut PgConnection, user_id_val: i32, new_role_val: &str) -> QueryResult<User> {
    diesel::update(users.find(user_id_val))
        .set(role.eq(new_role_val))
        .returning(User::as_returning())
        .get_result(conn)
}

/// Retrieves all registered users
pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users
        .order(id.asc())
        .select(User::as_select())
        .load(conn)
}

/// Counts the total number of users
pub fn count(conn: &mut PgConnection) -> QueryResult<i64> {
    users.count().get_result(conn)
}
