use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectRequest<'a> {
    #[serde(rename = "title")]
    pub title:  &'a str,
    #[serde(rename = "description")]
    pub description:  &'a str,
}