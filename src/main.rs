mod handlers;

use actix_web::{web, App, HttpServer};
use crate::handlers::general;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("running server on localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(general::hello)
            .service(general::about)
            .route("/health", web::get().to(general::health))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}