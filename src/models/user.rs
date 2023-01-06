use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    id: Uuid,
    username: String,
    login: String,
    secret: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub login: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn id_is_eq(&self, id: Uuid) -> bool {
        self.id.eq(&id)
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn id_as_ref(&self) -> &Uuid {
        &self.id
    }

    pub fn secret_as_ref(&self) -> &str {
        &self.secret
    }
}