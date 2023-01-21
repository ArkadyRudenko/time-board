use rocket::serde::json::Json;
use crate::models::user::{LoginError, RegistrationError, User};
use crate::routes::route_objects::login_request::LoginRequest;
use crate::routes::route_objects::login_response::LoginResponse;
use crate::routes::route_objects::registration_request::RegistrationRequest;
use crate::routes::route_objects::error_response::*;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub async fn login(
    maybe_login_request: Option<Json<LoginRequest<'_>>>
) -> Result<Json<LoginResponse>, ErrorResponse> {
    let call_chain =
        maybe_login_request.map(|r| User::login(r.login, r.password));

    return match call_chain {
        Some(Ok(token)) => Ok(Json(LoginResponse::from(token))),
        Some(Err(LoginError::NotFound)) => Err(ERROR_USER_NOT_FOUND),
        None => Err(ERROR_WRONG_REQUEST),
        _ => Err(ERROR_UNKNOWN),
    };
}

// TODO add string with error
#[post("/registration", format = "json", data = "<maybe_registration_request>")]
pub async fn registration(
    maybe_registration_request: Option<Json<RegistrationRequest<'_>>>
) -> Result<(), ErrorResponse<'_>> {

    let call_chain = maybe_registration_request.map(|user| {
        User::register(user.into_inner())
    });

    return match call_chain {
        Some(Ok(_)) => Ok(()),
        Some(Err(RegistrationError::LoginInUse)) => Err(ERROR_ALREADY_REGISTERED),
        Some(Err(RegistrationError::WeakPassword)) => Err(ERROR_WEAK_PASSWORD),
        Some(Err(RegistrationError::IncorrectLogin)) => Err(ERROR_INCORRECT_LOGIN),
        None => Err(ERROR_WRONG_REQUEST),
        _ => Err(ERROR_UNKNOWN),
    };
}
