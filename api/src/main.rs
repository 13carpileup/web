use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use querystring::stringify;

mod gent;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/process_auth")]
async fn process_auth() -> impl Responder {
    HttpResponse::Ok().body("pluh")
}


#[get("/authorize")]
async fn authorize() -> impl Responder {
    let client_id = "d65280f93fa443a3a8f14569b2b32ab9";
    let redirect_url = "http://127.0.0.1:8080/process_auth";
    let scope = "user-read-recently-played user-read-playback-state user-modify-playback-state user-read-currently-playing streaming app-remote-control";
    let url = "https://accounts.spotify.com/authorize";
    let queries = stringify(vec![
        ("response_type","code"),
        ("client_id", client_id),
        ("redirect_url", redirect_url),
        ("scope", scope),
        ("state", "meowmeowbeans")
    ]);

    let full_url = format!("{url}{queries}");

    HttpResponse::Ok().body(full_url)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(authorize)
            .service(process_auth)
            .service(gent::hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}