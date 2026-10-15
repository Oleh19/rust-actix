use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Bookmark {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub tag: String,
}
