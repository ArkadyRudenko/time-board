use std::time::Duration;
use argon2::password_hash::rand_core;
use diesel::RunQueryDsl;
use rand_core::RngCore;
use uuid::Uuid;
use crate::models::user::User;
use crate::schema::tokens;
use diesel::prelude::*;
use crate::db::token;

pub enum CreateTokenOutcome {
    Ok(String),
    Err,
}

#[derive(Queryable)]
pub struct Token {
    token: String,
    user_id: Uuid,
    created_at: Duration,
    last_used_at: Duration,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub user_id: &'a Uuid,
}

pub fn create_for_user(conn: &mut PgConnection, user: &User) -> CreateTokenOutcome {
    let mut token_bytes = [0u8; 32];
    rand_core::OsRng.fill_bytes(&mut token_bytes);
    let token_string = base64::encode(token_bytes);
    let token_entry = NewToken {
        token: &token_string,
        user_id: &user.id_as_ref(),
    };
    match diesel::insert_into(tokens::table)
        .values(token_entry)
        .execute(conn)
    {
        Ok(_) => CreateTokenOutcome::Ok(token_string),
        Err(e) => {
            eprintln!("Error inserting token: {}", e);
            CreateTokenOutcome::Err
        }
    }
}

// pub fn get_token(conn: &mut PgConnection, token: NewToken) -> Option<Token> {
//     return match tokens::table
//         .filter(tokens::token.eq(token.token))
//         .first(conn) {
//         Ok(token) => Some(token),
//         Err(_) => None,
//     };
// }