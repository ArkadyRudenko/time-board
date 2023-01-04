use crate::models::user::{NewUser, User};
use diesel::prelude::*;
use crate::db::{establish_connection, RegistrationOutcome};

pub fn registration_user(conn: &mut PgConnection, new_user: NewUser) -> RegistrationOutcome {
    use crate::schema::users;

    return match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn) {
        Ok(_) => RegistrationOutcome::Ok,
        Err(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            )) => RegistrationOutcome::AlreadyInUse,
        _ => RegistrationOutcome::Other,
    }
}