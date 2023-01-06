use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::models::user::{NewUser, User};
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;

pub enum GetUserOutcome {
    Some(User),
    None,
    Error,
}

pub fn select_user_by_id(conn: &mut PgConnection, id: Uuid) -> Option<User> {
    return match users::table
        .filter(users::id.eq(id))
        .first(conn) {
        Ok(user) => Some(user),
        Err(_) => None,
    };
}

pub fn select_user_by_username(conn: &mut PgConnection, username: &str) -> Option<User> {
    return match users::table
        .filter(users::username.eq(username))
        .first(conn) {
        Ok(user) => Some(user),
        Err(_) => None,
    };
}

pub fn with_credentials(conn: &mut PgConnection, login: &str, password: &str) -> GetUserOutcome {
    match users::table
        .filter(users::login.eq(login.to_lowercase()))
        .get_result::<User>(conn)
    {
        Ok(user) => {
            let argon2 = Argon2::default();
            if let Ok(parsed_hash) = PasswordHash::new(user.secret_as_ref()) {
                if argon2
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    GetUserOutcome::Some(user)
                } else {
                    GetUserOutcome::None
                }
            } else {
                GetUserOutcome::None
            }
        }
        Err(diesel::result::Error::NotFound) => GetUserOutcome::None,
        _ => GetUserOutcome::Error,
    }
}