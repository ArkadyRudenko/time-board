use rocket_contrib::json::Json;
use crate::handlers::authentication::login::{create_token, LoginError};
use crate::handlers::authentication::registration::{create_new_user, RegistrationError};
use crate::models::user::NewUser;
use crate::routes::route_objects::error_response::{ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN,
                                                   ERROR_USER_NOT_FOUND, ERROR_WEAK_PASSWORD,
                                                   ERROR_WRONG_REQUEST, ErrorResponse};
use crate::routes::route_objects::login_request::LoginRequest;
use crate::routes::route_objects::login_response::LoginResponse;
use crate::routes::route_objects::registration_request::RegistrationRequest;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub fn login<'r>(
    maybe_login_request: Option<Json<LoginRequest>>
) -> Result<Json<LoginResponse>, ErrorResponse<'r>> {
    let call_chain =
        maybe_login_request.map(|r| create_token(r.login, r.password));
    return match call_chain {
        Some(Ok(token)) => {
            let login_response = LoginResponse::from(token);
            let json_response = Json(login_response);
            Ok(json_response)
        }
        Some(Err(LoginError::NotFound)) => Err(ERROR_USER_NOT_FOUND),
        None => Err(ERROR_WRONG_REQUEST),
        _ => Err(ERROR_UNKNOWN),
    };
}

#[post(
"/registration",
format = "json",
data = "<maybe_registration_request>"
)]
pub fn registration<'r>(
    maybe_registration_request: Option<Json<RegistrationRequest>>
) -> Result<(), ErrorResponse<'r>> {

    let call_chain = maybe_registration_request.map(|user| {
        create_new_user(user.to_owned())
    });

    return match call_chain {
        Some(Ok(_)) => Ok(()),
        Some(Err(RegistrationError::LoginInUse)) => Err(ERROR_ALREADY_REGISTERED),
        Some(Err(RegistrationError::WeakPassword)) => Err(ERROR_WEAK_PASSWORD),
        None => Err(ERROR_WRONG_REQUEST),
        _ => Err(ERROR_UNKNOWN),
    };
}
