use std::time::Duration;
use uuid::Uuid;
use crate::models::task::Task;

pub struct Project {
    id: Uuid,
    title: String,
    description: String,
    tasks: Vec<Task>,
}

impl Project {
    pub fn get_global_time(&self) -> Duration {
        let mut result = Duration::default();

        for task in &self.tasks {
            result += task.get_global_time();
        }

        result
    }
}