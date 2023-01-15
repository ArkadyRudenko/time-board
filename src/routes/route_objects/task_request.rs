use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TaskRequest<'a> {
    #[serde(rename = "description")]
    pub description:  &'a str,
    #[serde(rename = "access_token")]
    pub access_token:  &'a str,
}