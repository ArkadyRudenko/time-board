use crate::models::project::Project;
use rocket::serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct ProjectResponse {
    title: String,
    description: String,
    uuid: String,
}

impl<'a> ProjectResponse {
    pub fn fromProject(project: &'a Project) -> Self {
        Self {
            title: project.title().to_string(),
            description: project.description().to_string(),
            uuid: project.uuid().to_string(),
        }
    }
}