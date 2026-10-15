use actix_web::{get, web, Responder};
use crate::models::Bookmark;

#[get("/bookmarks")]
pub async fn list_bookmarks( ) -> impl Responder {
    let bookmarks:Vec<Bookmark> = vec! [
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
            tag: String::from("example"),
        }
    ];
    
    web::Json(bookmarks)
}