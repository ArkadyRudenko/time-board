use crate::routes::route_objects::task_request::TaskRequest;
use rocket::serde::json::Json;
use crate::db::token::{SelectTokenOutCome, Token};
use crate::models::project::Project;
use crate::models::task::{NewTask, Task};
use crate::routes::route_objects::error_response::{ERROR_ACCESS_OUT_DATED, ERROR_PROJECT_NOT_FOUND, ERROR_TASK_NOT_ADDED, ERROR_USER_NOT_FOUND, ERROR_WRONG_REQUEST, ErrorResponse};
use diesel::result::Error;

#[post("/project/<project_id>/task?<access_token>", format = "json", data = "<maybe_task_request>")]
pub async fn create_task<'r>(
    project_id: &str,
    access_token: &str,
    maybe_task_request: Option<Json<TaskRequest<'r>>>
) -> Result<Json<String>, ErrorResponse<'r>> {

    return match Token::select(access_token) {
        SelectTokenOutCome::Some(_) => {
            match maybe_task_request {
                Some(request) => {
                    match Project::select(project_id) {
                        Ok(project) => {
                            match Task::insert(NewTask {
                                project_id: project.uuid().clone(),
                                description: request.description,
                            }) {
                                Ok(_) => Ok(Json(String::from("Task was added"))),
                                _ => Err(ERROR_TASK_NOT_ADDED)
                            }
                        }
                        NotFound => Err(ERROR_PROJECT_NOT_FOUND),
                    }
                }
                None => Err(ERROR_WRONG_REQUEST)
            }
        }
        SelectTokenOutCome::OutDated => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND),
    }
}

#[get("/project/<project_id>/tasks?<access_token>")]
pub async fn get_all_task<'r>(
    project_id: &str,
    access_token: &str,
) -> Result<Json<Vec<Task>>, ErrorResponse<'r>> {
    return match Token::select(access_token) {
        SelectTokenOutCome::Some(_) => {
            match Task::all(project_id) {
                Ok(tasks) => Ok(Json(tasks)),
                _ => Err(ERROR_PROJECT_NOT_FOUND),
            }
        }
        SelectTokenOutCome::OutDated => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND),
    }
}