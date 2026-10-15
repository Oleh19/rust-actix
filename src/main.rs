mod handlers;
mod models;
mod state;

use actix_web::{web, App, HttpServer};
use crate::handlers::{general, bookmarks};
use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState::new());
    
    println!("running server on localhost:8080");

    HttpServer::new(move || App::new()
            .app_data(state.clone())
            .service(general::hello)
            .service(general::about)
            .route("/health", web::get().to(general::health))
            .service(bookmarks::list_bookmarks)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}