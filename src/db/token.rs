use argon2::password_hash::rand_core;
use diesel::RunQueryDsl;
use rand_core::RngCore;
use uuid::Uuid;
use crate::models::user::User;
use crate::schema::tokens;
use diesel::prelude::*;
use crate::db::{establish_connection};

pub enum CreateTokenOutcome {
    Ok(String),
    Err,
}

#[derive(Queryable, Clone)]
pub struct Token {
    token: String,
    user_id: Uuid,
    created_at: std::time::SystemTime,
    last_used_at: std::time::SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "tokens"]
pub struct NewToken<'a> {
    pub token: &'a str,
    pub user_id: &'a Uuid,
}

impl Token {
    pub fn get_user_id(&self) -> Uuid {
        self.user_id.clone()
    }

    pub fn create_for_user(user: &User) -> CreateTokenOutcome {
        let mut token_bytes = [0u8; 32];
        rand_core::OsRng.fill_bytes(&mut token_bytes);
        let token_string = base64::encode(token_bytes);

        let token_entry = NewToken {
            token: &token_string,
            user_id: &user.id_as_ref(),
        };

        let conn = &mut establish_connection();

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

    // CDbsKFaGoN7CpKbafOWw0OCo I979NYkBl/9jaHuVGY=
    // CDbsKFaGoN7CpKbafOWw0OCo+I979NYkBl/9jaHuVGY=
    pub fn select(token: &str) -> Option<Token> {
        // TODO Why it replaces '+' to ' '?
        let new_token: String = token.chars().map(|ch| {
            if ch == ' ' { '+' } else { ch } }
        ).collect();

        return match tokens::table
            .filter(tokens::token.eq(new_token.as_str()))
            .first(&mut establish_connection()) {
            Ok(token) => Some(token),
            _ => None,
        };
    }
}
