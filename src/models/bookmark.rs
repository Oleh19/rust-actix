use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
pub struct Bookmark {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub tag: String,
}

#[derive(Deserialize)]
pub struct TagFilter {
    pub tag: Option<String>, 
}