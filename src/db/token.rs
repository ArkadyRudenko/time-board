use std::time::{Duration, SystemTime};
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

pub enum SelectTokenOutCome {
    Some(Token),
    Expired,
    None,
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
    pub fn select(token: &str) -> SelectTokenOutCome {
        // TODO Why it replaces '+' to ' '?
        let new_token: String = token.chars().map(|ch| {
            if ch == ' ' { '+' } else { ch }
        }
        ).collect();

        return match tokens::table
            .filter(tokens::token.eq(new_token.as_str()))
            .first::<Token>(&mut establish_connection()) {
            Ok(mut token) => {
                if token.is_expired() {
                    return SelectTokenOutCome::Expired;
                }
                token.update_last_used_at();

                SelectTokenOutCome::Some(token)
            }
            _ => SelectTokenOutCome::None,
        };
    }

    fn is_expired(&self) -> bool {
        static FREE_DAYS_IN_SECONDS: u64 = 259_200;
        static MAX_USED_TIME: Duration = Duration::new(FREE_DAYS_IN_SECONDS, 0);
        let total_used_time = self.last_used_at.elapsed().expect("get last used at");
        total_used_time > MAX_USED_TIME
    }

    fn update_last_used_at(&mut self) {
        diesel::update(crate::schema::tokens::table)
            .filter(tokens::token.eq(self.token.clone()))
            .set(tokens::last_used_at.eq(std::time::SystemTime::now()))
            .execute(&mut establish_connection());
    }
}
