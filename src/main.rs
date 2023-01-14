#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate core;

pub mod db;
pub mod models;
pub mod schema;
pub mod routes;

// #[derive(Database)]
// #[database("time-board")]
// struct DbConn(sqlx::PgPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .attach(DbConn::init())
        .mount(
        "/api-v1",
        routes![
                crate::routes::authentication::registration,
                crate::routes::authentication::login,
                crate::routes::projects::create_project,
                crate::routes::projects::get_all_projects,
            ],
    )
}
