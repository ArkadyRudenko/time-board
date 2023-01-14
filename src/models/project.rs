use std::str::FromStr;
use std::time::Duration;
use uuid::{Error, Uuid};
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::schema::projects;

pub enum GetProjectOutcome {
    Some(Project),
    None,
    Error,
}

pub enum InsertError {
    SomeError,
}

#[derive(Queryable)]
pub struct Project {
    id: Uuid,
    title: String,
    description: String,
    user_id: Uuid,
}

impl Project {
    pub fn get_global_time(&self) -> Duration {
        todo!()
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.id
    }

    pub fn select_projects_by_user_id(user_id: Uuid) -> Option<Vec<Project>> {
        return match projects::table
            .filter(projects::user_id.eq(user_id))
            .load::<Project>(&mut establish_connection()) {
            Ok(projects) => Some(projects),
            Err(_) => None,
        };
    }

    pub fn select_project(project_id: &str) -> Option<Project> {
        let project_id = Uuid::from_str(project_id);
        match project_id {
            Ok(project_uuid) => {
                return match projects::table
                    .filter(projects::id.eq(project_uuid))
                    .first(&mut establish_connection()) {
                    Ok(project) => Some(project),
                    Err(_) => None,
                };
            }
            Err(_) => None
        }

    }

    pub fn insert(new_project: NewProject) -> Result<(), InsertError> {
        match diesel::insert_into(crate::schema::projects::table)
            .values(&new_project)
            .get_result::<Project>(&mut establish_connection()) {
            Ok(_) => Ok(()),
            Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => Err(InsertError::SomeError),
            _ => Err(InsertError::SomeError),
        }
    }
}

#[derive(Insertable, Default)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub user_id: Uuid,
}

pub struct ProjectResponse {
    id: String,
    title: String,
    description: String,
    user_id: String,
}

impl From<Project> for ProjectResponse {
    fn from(value: Project) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title,
            description: value.description,
            user_id: value.user_id.to_string(),
        }
    }
}