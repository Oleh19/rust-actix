use actix_web::{get,post,put, delete, web, Responder, HttpResponse};
use crate::models::Bookmark;
use crate::models::bookmark::{NewBookmark, TagFilter};
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

#[post("/bookmarks")]
pub async fn create_bookmark(
    data: web::Data<AppState>,
    body: web::Json<NewBookmark>
) -> impl Responder {
    let new = body.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();

    let id = bookmarks.iter().map(|b| b.id)
        .max().unwrap_or(0) + 1;

    let bookmark = Bookmark {
        id,
        title: new.title,
        url: new.url,
        tag: new.tag,
    };

    (bookmarks).push(bookmark.clone());

    HttpResponse::Created().json(bookmark)
}

#[put("/bookmarks/{id}")]
pub async fn update_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    body: web::Json<NewBookmark>
) -> impl Responder {
    let id = path.into_inner();
    let changes = body.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();

    match bookmarks.iter_mut().find(|b| b.id == id) {
        Some(bookmark) => {
            bookmark.title = changes.title;
            bookmark.url = changes.url;
            bookmark.tag = changes.tag;
            HttpResponse::Ok().json(bookmark)
        }
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}

#[delete("/bookmarks/{id}")]
pub async fn delete_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>
) -> impl Responder {
    let id = path.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();

    match bookmarks.iter().position(|b| b.id == id) {
        Some(index) => {
            bookmarks.remove(index);
            HttpResponse::NoContent().finish()
        }
        None => HttpResponse::NotFound().body("Bookmark not found"),
    }
}