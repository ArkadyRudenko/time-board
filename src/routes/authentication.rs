use rocket_contrib::json::Json;
use crate::handlers::authentication::registration::{create_new_user, RegistrationError};
use crate::models::user::NewUser;
use crate::routes::route_objects::error_response::{ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN, ERROR_WEAK_PASSWORD, ERROR_WRONG_REQUEST, ErrorResponse};
use crate::routes::route_objects::registration_request::RegistrationRequest;

// TODO login request

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
