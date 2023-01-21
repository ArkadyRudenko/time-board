use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::models::user::{User};
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::establish_connection;
use crate::schema::users;

pub enum GetUserOutcome {
    Some(User),
    None,
    Error,
}

pub fn select_user_by_id(id: Uuid) -> GetUserOutcome {
    return match users::table
        .filter(users::id.eq(id))
        .first(&mut establish_connection()) {
        Ok(user) => GetUserOutcome::Some(user),
        Err(_) => GetUserOutcome::None,
        _ => GetUserOutcome::Error,
    };
}

pub fn select_user_by_username(username: &str) -> GetUserOutcome {
    return match users::table
        .filter(users::username.eq(username))
        .first(&mut establish_connection()) {
        Ok(user) => GetUserOutcome::Some(user),
        Err(_) => GetUserOutcome::None,
        _ => GetUserOutcome::Error,
    };
}

pub fn select_user_with_credentials(login: &str, password: &str) -> GetUserOutcome {
    match users::table
        .filter(users::login.eq(login.to_lowercase()))
        .get_result::<User>(&mut establish_connection())
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