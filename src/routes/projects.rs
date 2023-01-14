use std::str::FromStr;
use rocket::futures::StreamExt;
use rocket::serde::json::Json;
use serde::de::Unexpected::Str;
use uuid::{Error, Uuid};
use crate::db::establish_connection;
use crate::db::token::Token;
use crate::models::project::{InsertError, NewProject, Project};
use crate::models::user::LoginError;
use crate::routes::route_objects::error_response::{ERROR_INCORRECT_LOGIN, ERROR_PROJECTS_NOT_FOUND, ERROR_USER_NOT_FOUND, ERROR_WRONG_REQUEST, ErrorResponse};
use crate::routes::route_objects::project_request::ProjectRequest;
use crate::routes::route_objects::project_response::ProjectResponse;

#[post("/project", format = "json", data = "<maybe_project_request>")]
pub async fn create_project(
    maybe_project_request: Option<Json<ProjectRequest<'_>>>
) -> Result<Json<String>, ErrorResponse> {
    let mut new_project = NewProject::default();

    let call_chain = maybe_project_request.map(|p| {
        new_project.title = p.title;
        new_project.description = p.description;
        Token::select(p.user_token)
    });

    match call_chain {
        Some(Some(token)) => {
            new_project.user_id = token.get_user_id();
            match Project::insert(new_project) {
                Ok(_) => Ok(Json(String::from("Project was added"))),
                Err(_) => Err(ERROR_USER_NOT_FOUND),
            }
        }
        _ => Err(ERROR_USER_NOT_FOUND),
    }
}

#[get("/projects?<access_token>")]
pub async fn get_all_projects<'a>(
    access_token: &str
) -> Result<Json<Vec<ProjectResponse>>, ErrorResponse<'a>> {
    let token = Token::select(access_token);

    match token {
        Some(token) => {
            let projects = Project::select_projects_by_user_id(token.get_user_id());
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
        None => Err(ERROR_USER_NOT_FOUND)
    }
}

#[get("/project?<project_id>", format = "json", data = "<access_token>")]
pub async fn get_project(
    project_id: &str,
    access_token: Option<Json<String>>,
) -> Result<Json<ProjectResponse>, ErrorResponse> {
    return match access_token {
        Some(access_token) => {
            let token = Token::select(access_token.into_inner().as_str());

            match token {
                Some(_) => {
                    let project = Project::select_project(project_id);
                    match project {
                        Some(project) => {
                            Ok(Json(ProjectResponse::from_project(&project)))
                        }
                        None => Err(ERROR_PROJECTS_NOT_FOUND)
                    }
                }
                None => Err(ERROR_USER_NOT_FOUND)
            }
        }
        None => Err(ERROR_WRONG_REQUEST)
    }
}





















