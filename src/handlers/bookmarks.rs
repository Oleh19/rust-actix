use actix_web::{get, web, Responder};
use crate::models::Bookmark;
use crate::state::AppState;

#[get("/bookmarks")]
pub async fn list_bookmarks(
    data: web::Data<AppState>
) -> impl Responder {
    let bookmarks = data.bookmarks.lock().unwrap();

    web::Json(bookmarks.clone())
}