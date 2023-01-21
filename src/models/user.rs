use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::select_user::{GetUserOutcome, select_user_with_credentials};
use crate::db::register_user::{register_user, RegistrationOutcome};
use crate::db::token::{CreateTokenOutcome, Token};
use crate::routes::route_objects::registration_request::RegistrationRequest;
use crate::schema::users;

pub enum RegistrationError {
    LoginInUse,
    IncorrectLogin,
    WeakPassword,
    Other,
}

pub enum LoginError {
    NotFound,
    Other,
}

#[derive(Queryable)]
pub struct User {
    id: Uuid,
    username: String,
    login: String,
    secret: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub login: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn id_as_ref(&self) -> &Uuid {
        &self.id
    }

    pub fn secret_as_ref(&self) -> &str {
        &self.secret
    }

    pub fn register(user: RegistrationRequest) -> Result<(), RegistrationError> {
        if user.password.chars().count() < 8 {
            return Err(RegistrationError::WeakPassword);
        }
        if !Self::is_correct_login(user.login) {
            return Err(RegistrationError::IncorrectLogin);
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password_simple(user.password.as_bytes(), salt.as_ref())
            .unwrap();

        let new_user = NewUser {
            username: user.username,
            login: user.login,
            password: &password_hash.to_string(),
        };

        match register_user(new_user) {
            RegistrationOutcome::Ok => Ok(()),
            RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
            RegistrationOutcome::WeakPassword => Err(RegistrationError::WeakPassword),
            _ => Err(RegistrationError::Other),
        }
    }

    pub fn login(login: &str, password: &str) -> Result<String, LoginError> {
        match select_user_with_credentials(login, password) {
            GetUserOutcome::Some(user) => {
                match Token::create_for_user(&user) {
                    CreateTokenOutcome::Ok(token) => Ok(token),
                    CreateTokenOutcome::Err => Err(LoginError::Other),
                }
            }
            GetUserOutcome::None => Err(LoginError::NotFound),
            GetUserOutcome::Error => Err(LoginError::Other),
        }
    }

    fn is_correct_login(login: &str) -> bool {
        for x in login.chars() {
            if x.is_uppercase() {
                return false;
            }
        }
        true
    }
}