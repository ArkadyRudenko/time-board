use std::collections::{HashMap};
use uuid::Uuid;
use crate::db::establish_connection;
use crate::db::token::Token;
use diesel::prelude::*;

pub struct TokenManager {
    tokens: HashMap<String, Token>,
}

impl TokenManager {
    // TODO use Entry API
    pub fn contain_token(&mut self, token: &str) -> Option<Uuid> {
        use crate::schema::tokens;

        let token: String = token.chars().map(|ch| {
            if ch == ' ' { '+' } else { ch } }
        ).collect();

        match self.tokens.get(token.as_str()) {
            Some(token) => Some(token.get_user_id()),
            None => {
                return match tokens::table
                    .filter(tokens::token.eq(token.as_str()))
                    .first::<Token>(&mut establish_connection()) {
                    Ok(new_token) => {
                        let res = Some(new_token.get_user_id());
                        self.tokens.insert(token, new_token);
                        return res;
                    }
                    _ => None,
                };
            }
        }
    }
}