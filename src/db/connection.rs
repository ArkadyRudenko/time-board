use rocket_sync_db_pools::{database};
use crate::models::user::NewUser;

#[database("time-board")]
pub struct DbConn(rocket_sync_db_pools::diesel::PgConnection);

#[post("/")]
pub async fn insert_test(conn: DbConn) {
    conn.run(|conn| {
        let new_user = NewUser {
            username: "TEST",
            login: "TEST",
            password: "TEST",
        };

        // not compile =(
        // diesel::insert_into(crate::schema::users::table)
        //     .values(new_user)
        //     .execute(conn)

    }).await
}