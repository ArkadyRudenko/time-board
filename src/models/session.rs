use uuid::Uuid;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::serde::{Serialize};

use crate::db::establish_connection;
use crate::schema::sessions;

#[derive(Serialize, Debug)]
#[derive(Queryable)]
pub struct Session {
    id: Uuid,
    task_id: Uuid,
    start_task: std::time::SystemTime,
    end_task: std::time::SystemTime,
}

#[derive(Insertable, Default)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    task_id: Uuid,
}

impl Session {
    pub fn start(task_id: &Uuid) -> QueryResult<Session> {
        let new_session = NewSession{task_id: task_id.clone()};
        
        match diesel::insert_into(crate::schema::sessions::table)
            .values(&new_session)
            .get_result::<Session>(&mut establish_connection()) {
            Ok(session) => Ok(session),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(Error::NotFound),
            _ => Err(Error::NotFound),
        }
    }

    pub fn end(session_id: Uuid) -> Result<(), Error> {
        match diesel::update(crate::schema::sessions::table)
            .filter(sessions::id.eq(session_id))
            .set(sessions::end_task.eq(std::time::SystemTime::now()))
            .execute(&mut establish_connection()) {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(Error::NotFound),
            _ => Err(Error::NotFound),
        }
    }
}