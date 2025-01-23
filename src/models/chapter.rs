use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    pub chapter: Option<String>,
    pub title: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub type_field: String,
}