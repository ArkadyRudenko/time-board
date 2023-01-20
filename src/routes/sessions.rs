use std::str::FromStr;
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::db::token::Token;
use crate::routes::route_objects::error_response::{ERROR_SESSION_NOT_FOUND, ERROR_TASK_NOT_FOUND, ERROR_USER_NOT_FOUND, ErrorResponse};
use crate::models::session::Session;
use crate::models::task::Task;
use crate::routes::route_objects::access_token::AccessToken;

#[post("/project/<_>/task/<task_id>", format = "json", data = "<access_token>")]
pub async fn start_session<'r>(
    task_id: &str,
    access_token: Option<Json<AccessToken<'r>>>,
) -> Result<Json<Session>, ErrorResponse<'r>> {
    let call_chain = access_token.map(|token| {
        Token::select(token.access_token)
    });
    return match call_chain {
        Some(Some(_)) => {
            // TODO add check exist project and task
            match Task::select(task_id) {
                Ok(task) => {
                    match Session::start(task.get_id()) {
                        Ok(session) => {
                            Ok(Json(session))
                        }
                        _ => Err(ERROR_TASK_NOT_FOUND)
                    }
                },
                _ => Err(ERROR_TASK_NOT_FOUND)
            }
        },
        _ => Err(ERROR_USER_NOT_FOUND),
    };
}

#[post("/project/<_>/task/<_>/<session_id>", format = "json", data = "<access_token>")]
pub async fn end_session<'r>(
    session_id: &str,
    access_token: Option<Json<AccessToken<'r>>>,
) -> Result<Json<String>, ErrorResponse<'r>> {
    let call_chain = access_token.map(|token| {
        Token::select(token.access_token)
    });
    return match call_chain {
        Some(Some(_)) => {
            let session_uuid = Uuid::from_str(session_id);
            return match session_uuid {
                Ok(session_uuid) => {
                    match Session::end(session_uuid) {
                        Ok(()) => Ok(Json(String::from("Session ended"))),
                        _ => Err(ERROR_SESSION_NOT_FOUND),
                    }
                }
                _ => Err(ERROR_SESSION_NOT_FOUND),
            };
        },
        _ => Err(ERROR_USER_NOT_FOUND),
    };
}