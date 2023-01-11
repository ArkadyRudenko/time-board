use std::time::Duration;
use uuid::Uuid;
use diesel::prelude::*;
use crate::schema::projects;

pub enum GetProjectOutcome {
    Some(Project),
    None,
    Error,
}

#[derive(Queryable)]
pub struct Project {
    id: Uuid,
    title: String,
    pub description: String,
    user_id: Uuid,
}

impl Project {
    pub fn get_global_time(&self) -> Duration {
        todo!()
    }
}

impl Project {
    pub fn select_project_by_user_id(conn: &mut PgConnection, user_id: Uuid) -> Option<Project> {
        return match projects::table
            .filter(projects::user_id.eq(user_id))
            .first(conn) {
            Ok(project) => Some(project),
            Err(_) => None,
        };
    }
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