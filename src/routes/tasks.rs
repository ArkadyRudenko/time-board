use std::str::FromStr;
use crate::routes::route_objects::task_request::TaskRequest;
use rocket::serde::json::Json;
use serde::de::Unexpected::Str;
use uuid::Uuid;
use crate::db::token::Token;
use crate::models::project::Project;
use crate::models::task::{NewTask, Task};
use crate::routes::route_objects::error_response::{ERROR_PROJECT_NOT_FOUND, ERROR_TASK_NOT_ADDED, ERROR_USER_NOT_FOUND, ERROR_WRONG_REQUEST, ErrorResponse};

#[post("/task", format = "json", data = "<maybe_task_request>")]
pub async fn create_task(
    maybe_task_request: Option<Json<TaskRequest<'_>>>
) -> Result<Json<String>, ErrorResponse> {
    return match maybe_task_request {
        Some(request) => {
            match Token::select(request.access_token) {
                Some(_) => {
                    match Project::select_project(request.project_uuid) {
                        Some(project) => {
                            match Task::insert(NewTask {
                                project_id: project.uuid().clone(),
                                description: request.description,
                            }) {
                                Ok(_) => Ok(Json(String::from("Task was added"))),
                                InsertError => Err(ERROR_TASK_NOT_ADDED)
                            }
                        }
                        None => Err(ERROR_PROJECT_NOT_FOUND),
                    }
                }
                None => Err(ERROR_USER_NOT_FOUND),
            }
        }
        None => Err(ERROR_WRONG_REQUEST),
    }
}