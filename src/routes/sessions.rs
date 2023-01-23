use std::str::FromStr;
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::db::token::{SelectTokenOutCome, Token};
use crate::routes::route_objects::error_response::{ERROR_ACCESS_OUT_DATED, ERROR_DB_INSERT_ERROR, ERROR_INVALID_UUID_ERROR, ERROR_SESSION_NOT_FOUND, ERROR_TASK_NOT_FOUND, ERROR_USER_NOT_FOUND, ErrorResponse};
use crate::models::session::Session;
use crate::models::task::Task;

#[post("/project/<_>/task/<task_id>?<access_token>")]
pub async fn start_session<'r>(
    task_id: &str,
    access_token: &str,
) -> Result<Json<Session>, ErrorResponse<'r>> {
    return match Token::select(access_token) {
        SelectTokenOutCome::Some(_) => {
            match Task::select(task_id) {
                Ok(task) => {
                    match Session::start(task.get_id()) {
                        Ok(session) => {
                            Ok(Json(session))
                        }
                        _ => Err(ERROR_DB_INSERT_ERROR)
                    }
                },
                _ => Err(ERROR_TASK_NOT_FOUND)
            }
        }
        SelectTokenOutCome::Expired => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND),
    };
}

#[post("/project/<_>/task/<_>/<session_id>?<access_token>")]
pub async fn end_session<'r>(
    session_id: &str,
    access_token: &str,
) -> Result<Json<String>, ErrorResponse<'r>> {
    return match Token::select(access_token) {
        SelectTokenOutCome::Some(_) => {
            let session_uuid = Uuid::from_str(session_id);
            return match session_uuid {
                Ok(session_uuid) => {
                    match Session::end(session_uuid) {
                        Ok(()) => Ok(Json(String::from("Session ended"))),
                        _ => Err(ERROR_SESSION_NOT_FOUND),
                    }
                }
                _ => Err(ERROR_INVALID_UUID_ERROR),
            };
        }
        SelectTokenOutCome::Expired => Err(ERROR_ACCESS_OUT_DATED),
        SelectTokenOutCome::None => Err(ERROR_USER_NOT_FOUND),
    };
}