use crate::models::user::{NewUser, User};
use diesel::prelude::*;
use uuid::Uuid;

pub fn select_user_by_id(conn: &mut PgConnection, id: Uuid) -> Option<User> {
    use crate::schema::users;

    return match users::table
        .filter(users::id.eq(id))
        .first(conn) {
        Ok(user) => Some(user),
        Err(_) => None,
    };
}

pub fn select_user_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
    use crate::schema::users;

    return match users::table
        .filter(users::username.eq(username))
        .first(conn) {
        Ok(user) => Some(user),
        Err(_) => None,
    };
}