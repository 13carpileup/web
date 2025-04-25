use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use auth::DbPool;
use querystring::stringify;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::env;
use dotenv;

mod gent;
mod spotify;
mod auth;

#[get("/pause")]
async fn pause(pool: web::Data<DbPool>) -> impl Responder {
    spotify::pause_playback(pool).await;

    HttpResponse::Ok().body("paused?")
}

#[derive(Serialize, Deserialize)]
pub struct Uri {
    uri: String
}

#[get("/add_song")]
async fn add_song(uri: web::Query<Uri>, pool: web::Data<DbPool>) -> impl Responder {
    println!("URI: {u}", u=uri.uri.clone());
    
    spotify::add_song(uri.uri.clone(), pool).await;

    HttpResponse::Ok().body("added?")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize)]
pub struct Search {
    search_term: String
}

#[get("/search")]
async fn search(search: web::Query<Search>, pool: web::Data<DbPool>) -> impl Responder {
    let res = spotify::search(&search.search_term, pool).await;
    println!("Searching for {s}", s = search.search_term);
    
    HttpResponse::Ok().body(json!(res).to_string())
}

#[derive(Deserialize)]
struct Info {
    code: String,
    state: String
}

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    access_token: String,
    refresh_token: Option<String>,
    scope: String,
    expires_in: u64,
    token_type: String
}

#[get("/authorize")]
async fn authorize() -> impl Responder {
    dotenv::dotenv().ok();

    let domain = env::var("URL").unwrap();
    let client_id = "d65280f93fa443a3a8f14569b2b32ab9";
    let redirect_url = format!("{domain}process_auth");
    let scope = "user-read-recently-played user-read-playback-state user-modify-playback-state user-read-currently-playing streaming app-remote-control";
    let url = "https://accounts.spotify.com/authorize";
    let queries = stringify(vec![
        ("response_type","code"),
        ("client_id", client_id),
        ("redirect_uri", &redirect_url),
        ("scope", scope),
        ("state", "meowmeowbeans")
    ]);

    let full_url = format!("{url}?{queries}");

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
    let db_path = "main.db";

    let pool = auth::initialize_db_pool(&db_path)
        .expect("Failed to initialize database pool.");
    
    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().send_wildcard();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(authorize)
            .service(auth::process_auth)
            .service(pause)
            .service(add_song)
            .service(search)
            .service(gent::hello)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}