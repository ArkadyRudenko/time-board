use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::serde::{Serialize};
use crate::models::session::Session;
use crate::db::establish_connection;
use crate::models::project::InsertError;

use crate::schema::tasks;

pub enum StartError {
    SomeError
}

#[derive(Queryable, Serialize)]
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
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_time(&self) -> Option<Duration> {
        let sessions = Session::all(&self.id);

        match sessions {
            Ok(sessions) => {
                let times: Vec<Duration> = sessions.iter().map(|s| {
                    let start = s.start_task.elapsed().unwrap();
                    let end = s.end_task.elapsed().unwrap();
                    start - end
                }).collect();
                let mut res = Duration::default();
                for t in times.into_iter() {
                    res += t;
                }
                Some(res)
            }
            _ => None
        }
    }

    pub fn insert(new_task: NewTask) -> Result<(), InsertError> {
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

    pub fn select(task_id: &str) -> QueryResult<Task> {
        let task_id = Uuid::from_str(task_id);
        return match task_id {
            Ok(task_uuid) => {
                tasks::table
                    .filter(tasks::id.eq(task_uuid))
                    .first(&mut establish_connection())
            }
            Err(_) => Err(Error::NotFound)
        }
    }

    pub fn all(project_id: &str) -> QueryResult<Vec<Task>> {
        let project_id = Uuid::from_str(project_id);
        match project_id {
            Ok(project_id) => {
                return match tasks::table
                    .filter(tasks::project_id.eq(project_id))
                    .load::<Task>(&mut establish_connection()) {
                    Ok(tasks) => Ok(tasks),
                    Err(_) => Err(Error::NotFound),
                };
            }
            Err(_) => Err(Error::NotFound)
        }
    }
}