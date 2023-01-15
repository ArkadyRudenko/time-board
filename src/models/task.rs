use uuid::Uuid;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::project::InsertError;

use crate::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    id: Uuid,
    description: String,
    project_id: Uuid,
}

#[derive(Insertable, Default)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub project_id: Uuid,
    pub description: &'a str,
}

impl Task {
    pub fn insert(new_task: NewTask) -> Result<(), InsertError>  {
        match diesel::insert_into(crate::schema::tasks::table)
            .values(&new_task)
            .get_result::<Task>(&mut establish_connection()) {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(InsertError::SomeError),
            _ => Err(InsertError::SomeError),
        }
    }
}