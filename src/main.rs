#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use crate::routes::TimesheetsRoutesInitialized;

pub mod db;
pub mod models;
pub mod schema;
pub mod handlers;
pub mod routes;

fn main() {
    rocket::ignite()
        .mount_timesheet_routes()
        .launch();
}