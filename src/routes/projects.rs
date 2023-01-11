use std::str::FromStr;
use rocket::serde::json::Json;
use uuid::Uuid;
use serde::Deserialize;
use crate::db::establish_connection;
use crate::models::project::{Project};
use crate::routes::route_objects::error_response::{ERROR_USER_NOT_FOUND, ErrorResponse};

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectRequest<'a> {
    #[serde(rename = "user_id")]
    pub user_id: &'a str,
}

#[get("/project", format = "json", data = "<maybe_project_request>")]
pub async fn get_project(
    maybe_project_request: Option<Json<ProjectRequest<'_>>>
) -> Result<Json<String>, ErrorResponse<'_>> {

    let call_chain = maybe_project_request.map(|user_id| {
        let connection = &mut establish_connection();
        let user_id = Uuid::from_str(user_id.to_owned().user_id);
        Project::select_project_by_user_id(connection, user_id.unwrap())
    });

    return match call_chain {
        Some(value) => {
            match value {
                Some(project) => {
                    let json = Json(project.description.clone());
                    return Ok(json);
                }
                None => Err(ERROR_USER_NOT_FOUND)
            }
        }
        None => {
            Err(ERROR_USER_NOT_FOUND)
        }
    }
}