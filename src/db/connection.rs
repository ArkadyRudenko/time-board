use rocket_sync_db_pools::{database};
use rocket_sync_db_pools::diesel::RunQueryDsl;
use rocket_sync_db_pools::diesel::Insertable;

// not compile =(
/*

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub login: &'a str,
    pub password: &'a str,
}

#[database("time-board")]
pub struct DbConn(diesel::PgConnection);

#[post("/")]
pub async fn insert_test(conn: DbConn) {
    conn.run(|conn| {
        let new_user = NewUser {
            username: "TEST",
            login: "TEST",
            password: "TEST",
        };
        diesel::insert_into(crate::schema::users::table)
            .values(new_user)
            .execute(&*conn)
    }).await
}

*/