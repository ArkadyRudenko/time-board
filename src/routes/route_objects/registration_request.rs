use rocket::serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct RegistrationRequest<'a> {
    #[serde(rename = "username")]
    pub username: &'a str,
    #[serde(rename = "login")]
    pub login: &'a str,
    #[serde(rename = "password")]
    pub password: &'a str,
}
