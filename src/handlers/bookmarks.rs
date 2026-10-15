use actix_web::{get, web, Responder, HttpResponse};
use crate::models::Bookmark;
use crate::models::bookmark::TagFilter;
use crate::state::AppState;

#[get("/bookmarks")]
pub async fn list_bookmarks(
    data: web::Data<AppState>,
    query: web::Query<TagFilter>,
) -> impl Responder {
    let bookmarks = data.bookmarks.lock().unwrap();

    let filtered: Vec<Bookmark> = bookmarks
        .iter()
        .filter(|b| match &query.tag {
            Some(tag) => &b.tag == tag,
            None => true,
        })
        .cloned()
        .collect();
    web::Json(filtered)
}

#[get("/bookmarks/{id}")]
pub async fn get_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>
) -> impl Responder {
    let id = path.into_inner();
    let bookmarks = data.bookmarks.lock().unwrap();

    match bookmarks.iter().find(|b | b.id == id) {
        Some(bookmark) => HttpResponse::Ok()
            .json(bookmark),
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}