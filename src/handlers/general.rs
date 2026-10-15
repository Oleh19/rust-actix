use actix_web::{get, HttpResponse, Responder };

#[get("/")]
pub async fn hello() -> impl Responder {
    "Welcome to Actix-web!"
}

#[get("/about")]
pub async fn about() -> impl Responder {
    "Something about me"
}

pub async fn health() -> impl Responder {
    "Api is running"
}
