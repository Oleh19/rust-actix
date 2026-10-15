use std::sync::Mutex;

use crate::models::Bookmark;

pub struct AppState {
    pub bookmarks: Mutex<Vec<Bookmark>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            bookmarks: Mutex::new(
                vec! [
                    Bookmark {
                        id: 1,
                        title: String::from("Bookmark"),
                        url: String::from("example.com"),
                        tag: String::from("example"),
                    },

                    Bookmark {
                        id: 2,
                        title: String::from("Bookmark"),
                        url: String::from("example.com"),
                        tag: String::from("horror"),
                    }
                ]
               )
            
        }
    } 
}