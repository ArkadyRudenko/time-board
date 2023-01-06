use uuid::Uuid;
use crate::models::task::Task;

pub struct Project {
    id: Uuid,
    title: String,
    description: String,
    tasks: Vec<Task>,
}