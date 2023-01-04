use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use crate::db::registration_user::registration_user;
use crate::db::{establish_connection, RegistrationOutcome};
use crate::models::user::NewUser;
use crate::routes::route_objects::registration_request::RegistrationRequest;

pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    Other,
}

pub fn create_new_user(user: RegistrationRequest) -> Result<(), RegistrationError> {
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

    let connection = &mut establish_connection();

    match registration_user(connection, new_user) {
        RegistrationOutcome::Ok => Ok(()),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::WeakPassword => Err(RegistrationError::WeakPassword),
        _ => Err(RegistrationError::Other),
    }
}
