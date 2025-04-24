use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/test")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("meow.//")
}
