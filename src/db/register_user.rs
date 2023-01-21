use crate::models::user::{NewUser, User};
use diesel::prelude::*;
use crate::db::{establish_connection};

pub enum RegistrationOutcome {
    Ok,
    AlreadyInUse,
    WeakPassword,
    Other,
}

pub fn register_user(new_user: NewUser) -> RegistrationOutcome {
    return match diesel::insert_into(crate::schema::users::table)
        .values(&new_user)
        .get_result::<User>(&mut establish_connection()) {
        Ok(_) => RegistrationOutcome::Ok,
        Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => RegistrationOutcome::AlreadyInUse,
        _ => RegistrationOutcome::Other,
    }
}