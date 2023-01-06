use rocket::Rocket;

use crate::routes::route_objects::error_response::{
    ERROR_UNAUTHORIZED, ERROR_UNKNOWN, ErrorResponse,
};

use index::test;

use crate::routes::{index, TimesheetsRoutesInitialized};

impl TimesheetsRoutesInitialized for Rocket {
    fn mount_timesheet_routes(self) -> Self {
        self.mount(
            "/api-v1",
            routes![
                index::test,
                crate::routes::authentication::registration,
                crate::routes::authentication::login
            ],
        )
            .register(catchers![unauthorized, unknown])
    }
}

#[catch(401)]
fn unauthorized<'r>() -> ErrorResponse<'r> {
    return ERROR_UNAUTHORIZED;
}
#[catch(500)]
fn unknown<'r>() -> ErrorResponse<'r> {
    return ERROR_UNKNOWN;
}
