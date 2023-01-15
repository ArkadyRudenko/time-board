use std::str::FromStr;
use crate::routes::route_objects::task_request::TaskRequest;
use rocket::serde::json::Json;
use serde::de::Unexpected::Str;
use uuid::Uuid;
use crate::db::token::Token;
use crate::models::project::Project;
use crate::models::task::{NewTask, Task};
use crate::routes::route_objects::access_token::AccessToken;
use crate::routes::route_objects::error_response::{ERROR_PROJECT_NOT_FOUND, ERROR_TASK_NOT_ADDED, ERROR_USER_NOT_FOUND, ERROR_WRONG_REQUEST, ErrorResponse};

#[post("/project/<project_id>/task", format = "json", data = "<maybe_task_request>")]
pub async fn create_task<'r>(
    project_id: &str,
    maybe_task_request: Option<Json<TaskRequest<'r>>>
) -> Result<Json<String>, ErrorResponse<'r>> {
    return match maybe_task_request {
        Some(request) => {
            match Token::select(request.access_token) {
                Some(_) => {
                    match Project::select_project(project_id) {
                        Some(project) => {
                            match Task::insert(NewTask {
                                project_id: project.uuid().clone(),
                                description: request.description,
                            }) {
                                Ok(_) => Ok(Json(String::from("Task was added"))),
                                _ => Err(ERROR_TASK_NOT_ADDED)
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

#[get("/project/<project_id>/tasks", format = "json", data = "<maybe_task_request>")]
pub async fn get_all_task<'r>(
    project_id: &str,
    maybe_task_request: Option<Json<AccessToken<'r>>>,
) -> Result<Json<Vec<Task>>, ErrorResponse<'r>> {
    // TODO access_token
    match Task::all(project_id) {
        Ok(tasks) => Ok(Json(tasks)),
        _ => Err(ERROR_PROJECT_NOT_FOUND),
    }
}