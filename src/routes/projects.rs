use std::time::Duration;
use rocket::serde::json::Json;
use crate::db::token::{SelectTokenOutCome, Token};
use crate::models::project::{NewProject, Project};
use crate::routes::route_objects::error_response::{ERROR_ACCESS_OUT_DATED, ERROR_PROJECTS_NOT_FOUND, ERROR_USER_NOT_FOUND, ERROR_WRONG_REQUEST, ErrorResponse};
use crate::routes::route_objects::project_request::ProjectRequest;
use crate::routes::route_objects::project_response::ProjectResponse;
use diesel::result::Error;

#[post("/project?<access_token>", format = "json", data = "<maybe_project_request>")]
pub async fn create_project<'r>(
    access_token: &str,
    maybe_project_request: Option<Json<ProjectRequest<'r>>>,
) -> Result<Json<String>, ErrorResponse<'r>> {
    match Token::select(access_token) {
        SelectTokenOutCome::Some(token) => {
            let mut new_project = NewProject::default();

            let call_chain = maybe_project_request.map(|p| {
                new_project.title = p.title;
                new_project.description = p.description;
                new_project
            });

            match call_chain {
                Some(mut new_project) => {
                    new_project.user_id = token.get_user_id();
                    match Project::insert(new_project) {
                        Ok(_) => Ok(Json(String::from("Project was added"))),
                        Err(_) => Err(ERROR_USER_NOT_FOUND),
                    }
                }
                _ => Err(ERROR_WRONG_REQUEST),
            }
        }
        SelectTokenOutCome::Expired => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND)
    }
}

#[get("/projects?<access_token>")]
pub async fn get_all_projects<'a>(
    access_token: &str
) -> Result<Json<Vec<ProjectResponse>>, ErrorResponse<'a>> {
    let token = Token::select(access_token);

    match token {
        SelectTokenOutCome::Some(token) => {
            let projects = Project::select_projects_with_user_id(token.get_user_id());
            match projects {
                Some(projects) => {
                    let projects_responses = projects
                        .iter()
                        .map(|p| ProjectResponse::from_project(p)).collect();

                    return Ok(Json(projects_responses));
                }
                None => Err(ERROR_PROJECTS_NOT_FOUND)
            }
        }
        SelectTokenOutCome::Expired => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND),
    }
}

#[get("/project/<project_id>?<access_token>")]
pub async fn get_project<'r>(
    project_id: &str,
    access_token: &str,
) -> Result<Json<ProjectResponse>, ErrorResponse<'r>> {
    match Token::select(access_token) {
        SelectTokenOutCome::Some(_) => {
            let project = Project::select(project_id);
            match project {
                Ok(project) => {
                    Ok(Json(ProjectResponse::from_project(&project)))
                }
                NotFound => Err(ERROR_PROJECTS_NOT_FOUND)
            }
        }
        SelectTokenOutCome::Expired => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND)
    }
}

#[get("/project/<project_id>/time?<access_token>")]
pub async fn get_project_time<'r>(
    project_id: &str,
    access_token: &str,
) -> Result<Json<Duration>, ErrorResponse<'r>> {
    return match Token::select(access_token) {
        SelectTokenOutCome::Some(_) => {
            match Project::get_all_time(project_id) {
                Ok(time) => Ok(Json(time)),
                _ => Err(ERROR_PROJECTS_NOT_FOUND),
            }
        }
        SelectTokenOutCome::Expired => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND)
    };
}





















