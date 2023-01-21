use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TaskRequest<'a> {
    #[serde(rename = "description")]
    pub description:  &'a str,
}