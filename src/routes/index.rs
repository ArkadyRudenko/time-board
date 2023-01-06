use crate::db::establish_connection;
use crate::db::registration_user::registration_user;
use crate::db::get_user::select_user_by_username;
use crate::models::user::NewUser;

#[get("/")]
pub fn test() -> &'static str {
    "Hello, world!"
}