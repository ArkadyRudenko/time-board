#[macro_use] extern crate rocket;

pub mod db;
pub mod models;
pub mod schema;
pub mod routes;
pub mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api-v1",
            routes![
                crate::routes::authentication::registration,
                crate::routes::authentication::login,
                crate::routes::projects::create_project,
                crate::routes::projects::get_all_projects,
                crate::routes::projects::get_project,
                crate::routes::tasks::create_task,
                crate::routes::sessions::start_session,
                crate::routes::sessions::end_session,
                crate::routes::projects::get_project_time,
                crate::routes::tasks::get_all_task,
            ],
        )
}
