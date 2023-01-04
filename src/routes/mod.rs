pub mod route_objects;
pub mod authentication;
pub mod routes_setup;
pub mod index;

pub trait TimesheetsRoutesInitialized {
    fn mount_timesheet_routes(self) -> Self;
}
