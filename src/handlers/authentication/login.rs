
use crate::db::establish_connection;
use crate::db::get_user::{GetUserOutcome, with_credentials};
use crate::db::token::{create_for_user, CreateTokenOutcome};

pub enum LoginError {
    NotFound,
    Other,
}

pub fn create_token(
    login: &str,
    password: &str
) -> Result<String, LoginError> {

    let connection = &mut establish_connection();

    match with_credentials(connection, login, password) {
        GetUserOutcome::Some(user) => {
            match create_for_user(connection, &user) {
                CreateTokenOutcome::Ok(token) => Ok(token),
                CreateTokenOutcome::Err => Err(LoginError::Other),
            }
        }
        GetUserOutcome::None => Err(LoginError::NotFound),
        GetUserOutcome::Error => Err(LoginError::Other),
    }
}
