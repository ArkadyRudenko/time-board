use rocket::serde::{Deserialize};

#[derive(Deserialize)]
pub struct AccessToken<'a> {
    #[serde(rename = "access_token")]
    pub access_token: &'a str,
}
