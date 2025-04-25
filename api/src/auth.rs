use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use querystring::stringify;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use dotenv::dotenv;
use base64::prelude::*;
use rusqlite::{Connection, Result, params};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::time::{SystemTime, UNIX_EPOCH};

pub type DbPool = Pool<SqliteConnectionManager>;

pub struct Client {
    client_id: String,
    client_secret: String
}

pub struct TokenRow {
    access_token: String,
    refresh_token: String,
    time: u64
}

pub fn get_client_data() -> Client {
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").unwrap();
    let client_secret = env::var("CLIENT_SECRET").unwrap();

    Client {
        client_id,
        client_secret
    }
}

pub fn initialize_db_pool(db_path: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager)?; 

    let conn = pool.get()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
            access_token TEXT PRIMARY KEY,
            refresh_token TEXT,
            time INTEGER
    )",
    ()
    ).unwrap();

    Ok(pool)
}

pub async fn refresh_token(refresh_token: &str) -> super::TokenData {
    let auth = get_client_authorization();

    let params = [("grant_type", "refresh_token"), ("refresh_token", refresh_token)];

    let client = reqwest::Client::new();
    let res = client.post("https://accounts.spotify.com/api/token")
        .form(&params)
        .header("Authorization", auth)
        .send()
        .await
        .unwrap();

    let body: super::TokenData = serde_json::from_str(&res.text().await.unwrap()).unwrap();
    return body;
} 

pub fn get_client_authorization() -> String {
    let client_data = get_client_data();
    let preb64 = format!("{cl}:{cs}", cl = client_data.client_id, cs = client_data.client_secret);

    let postb64 = BASE64_STANDARD.encode(preb64);

    let auth = format!("Basic {postb64}");

    println!("auth: {auth}");

    auth
}

pub async fn get_token_authoritzation(pool: web::Data<DbPool>) -> String {
    let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let hour_ago = epoch_time - 3600;

    println!("HOUR AGO {hour_ago}");
    

    let conn = pool.get().unwrap();

    let res: i64 = conn.query_row("SELECT COUNT(*) FROM tokens", [], |row| row.get(0)).unwrap();
    // gets number of rows/tokens available

    if res > 0 {
        let mut stmt = conn.prepare("SELECT * FROM tokens WHERE time < ?").unwrap();

        let t_iter = stmt.query_map([hour_ago], |row| {
            Ok(TokenRow {
                access_token: row.get(0).unwrap(),
                refresh_token: row.get(1).unwrap(),
                time: row.get(2).unwrap()
            })
        }).unwrap();

        for t in t_iter {
            let row = t.unwrap();

            let new_access = refresh_token(&row.refresh_token).await;

            conn.execute("INSERT INTO tokens VALUES (?1, ?2, ?3)", params![new_access.access_token, new_access.refresh_token, epoch_time]).unwrap();
        }

        let access_token: String = conn.query_row("SELECT * FROM tokens", [], |row| row.get(0)).unwrap();

        return format!("Bearer {access_token}");
    }

    conn.execute("DELETE FROM tokens WHERE time < ?1", [hour_ago]).unwrap();

    println!("{res} NUMBER OF TOKENS HERE!");


    "meow".to_string()
}

#[get("/process_auth")]
pub async fn process_auth(
    info: web::Query<super::Info>,
    pool: web::Data<DbPool>
) -> impl Responder {
    println!("lovely state: {}", info.state);
    dotenv::dotenv().ok();

    let domain = env::var("URL").unwrap();

    let auth = get_client_authorization();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &info.code), 
        ("redirect_uri", &format!("{domain}process_auth")),
    ];

    let client = reqwest::Client::new();
    let res = client.post("https://accounts.spotify.com/api/token")
        .form(&params)
        .header("Authorization", auth)
        .send()
        .await;

    match res {
        Ok(e) => {
            if e.status().is_success() {
                let body: super::TokenData = serde_json::from_str(&e.text().await.unwrap()).unwrap();
                let access_token = body.access_token.clone();
                let refresh_token = body.refresh_token.clone();

                let epoch_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let conn = pool.get().unwrap();

                conn.execute("INSERT INTO tokens VALUES (?1, ?2, ?3)", params![access_token, refresh_token, epoch_time]).unwrap();
            } else {
                let status_code = e.status();
                let error_body = e.text().await.unwrap_or_else(|_| "sad".to_string());
                println!("LMFAFOO!!! {error_body}");
                println!("{status_code}");
                return HttpResponse::InternalServerError().body(error_body);
            }
        },
        Err(e) => {
            println!("idk brub");
            return HttpResponse::InternalServerError().body("Request to spotify failed. Sad!")
        }
    }

    HttpResponse::Ok().body(info.code.clone())
}