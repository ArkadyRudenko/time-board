use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::{Request, Response};
use rocket::serde::json::Json;

#[derive(Copy, Clone, Debug)]
pub struct ErrorResponse<'a> {
    cause: &'a str,
    pub status: Status,
}

impl<'r> Responder<'r, 'r> for ErrorResponse<'r> {
    fn respond_to(self, request: &Request) -> Result<'r> {
        if let Ok(response) = Json(json!({"error": self.cause})).respond_to(request) {
            Response::build_from(response)
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .ok()
        }
    }
}

// common errors
pub const ERROR_UNKNOWN: ErrorResponse = ErrorResponse {
    cause: "unknown",
    status: Status::InternalServerError,
};
pub const ERROR_WRONG_REQUEST: ErrorResponse = ErrorResponse {
    cause: "wrong_request",
    status: Status::BadRequest,
};
pub const ERROR_UNAUTHORIZED: ErrorResponse = ErrorResponse {
    cause: "unauthorized",
    status: Status::Unauthorized,
};

// login error
pub const ERROR_USER_NOT_FOUND: ErrorResponse = ErrorResponse {
    cause: "user_not_found",
    status: Status::BadRequest,
};

// registration error
pub const ERROR_WEAK_PASSWORD: ErrorResponse = ErrorResponse {
    cause: "weak_password",
    status: Status::BadRequest,
};
pub const ERROR_ALREADY_REGISTERED: ErrorResponse = ErrorResponse {
    cause: "already_registered",
    status: Status::BadRequest,
};
pub const ERROR_INCORRECT_LOGIN: ErrorResponse = ErrorResponse {
    cause: "login has uppercase characters",
    status: Status::BadRequest,
};

// projects error
pub const ERROR_PROJECTS_NOT_FOUND: ErrorResponse = ErrorResponse {
    cause: "there aren`t projects",
    status: Status::BadRequest,
};

pub const ERROR_PROJECT_NOT_FOUND: ErrorResponse = ErrorResponse {
    cause: "there asn`t project",
    status: Status::BadRequest,
};

// tasks error
pub const ERROR_TASK_NOT_ADDED: ErrorResponse = ErrorResponse {
    cause: "task wasn`t added",
    status: Status::BadRequest,
};

pub const ERROR_TASK_NOT_FOUND: ErrorResponse = ErrorResponse {
    cause: "task was not found",
    status: Status::BadRequest,
};

// session error
pub const ERROR_SESSION_NOT_FOUND: ErrorResponse = ErrorResponse {
    cause: "session was not found",
    status: Status::BadRequest,
};