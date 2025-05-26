use actix_web::web;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use base64::prelude::*;
use reqwest;
use crate::auth::{get_token_authoritzation, DbPool};
use std::fs::File;

use super::auth;


#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: Option<String>, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    id: String,
    name: String,
    #[serde(rename = "external_urls")] 
    external_urls: ExternalUrls,
    href: String, 
    #[serde(rename = "type")]
    artist_type: String, 
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlbumBrief { 
    id: String,
    name: String,
    #[serde(rename = "album_type")]
    album_type: String,
    #[serde(rename = "external_urls")]
    external_urls: ExternalUrls,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct TrackItem {
    album: AlbumBrief, 
    artists: Vec<Artist>, 
    available_markets: Option<Vec<String>>,
    disc_number: u32,
    duration_ms: u64,
    explicit: bool,
    #[serde(rename = "external_urls")]
    external_urls: ExternalUrls,
    href: String, 
    id: String,
    is_local: bool,
    is_playable: Option<bool>, 
    name: String,
    popularity: u64,
    preview_url: Option<String>, 
    track_number: u32,
    #[serde(rename = "type")]
    item_type: String, 
    uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TracksPagingObject {
    href: String,
    items: Vec<TrackItem>, 
    limit: u32,
    next: Option<String>,
    offset: u32,
    previous: Option<String>, 
    total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    tracks: TracksPagingObject,
}

#[derive(Serialize, Deserialize)]
pub struct Song {
    artists: Vec<String>,
    id: String,
    link: String,
    name: String,
    popularity: u64,
}

pub async fn pause_playback(pool: web::Data<DbPool>) {
    let auth = auth::get_token_authoritzation(pool).await;

    let client = reqwest::Client::new();
    let res = client.put("https://api.spotify.com/v1/me/player/pause")
        .header("Authorization", auth)
        .header("Content-Length", 0)
        .send()
        .await;

    let code = res.unwrap().status();

    println!("status of pause: {code}");
}

pub async fn search(search_term: &str, pool: web::Data<DbPool>) -> Vec<TrackItem> {
    let auth = auth::get_token_authoritzation(pool).await;

    let params = vec![
        ("q", search_term), 
        ("type", "track")
    ];

    let client = reqwest::Client::new();
    let res = client.get("https://api.spotify.com/v1/search")
        .query(&params)
        .header("Authorization", auth)
        .send()
        .await;

    match res {
        Ok(e) => {
            if !e.status().is_success() {
                println!("Failed with error code {e}", e = e.status());
                return vec![];
            }

            let results: SearchResult = serde_json::from_str::<SearchResult>(&e.text().await.unwrap()).unwrap();

            return results.tracks.items;
        }

        Err(v) => {
            println!("Error searching!");
            return vec![];
        }
    }

    vec![]
}

use std::fs::OpenOptions;
use std::io::Write;
use chrono;

pub async fn add_song(uri: String, pool: web::Data<DbPool>) {
    let auth = auth::get_token_authoritzation(pool).await;

    let params = vec![
        ("uri", uri.clone()),
        ("device_id", "c8fcbcc4ad03082bfdb03218cf49746cf08e87fe".to_string())
    ];

    let client = reqwest::Client::new();
    let res = client.post("https://api.spotify.com/v1/me/player/queue")
        .query(&params)
        .header("Authorization", auth.clone())
        .header("Content-Length", 0)
        .send()
        .await
        .unwrap();

    let mut data_file = OpenOptions::new()
        .append(true)
        .open("log.txt")
        .expect("cannot open file");

    let date = chrono::offset::Local::now();

    data_file
        .write(format!("{date}: added {uri}").as_bytes())
        .expect("write failed");

}

#[derive(Serialize, Deserialize)]
pub struct Device {
    id: String,
    is_active: bool,
    name: String,
    song_name: Option<String>,
    song_artist: Option<String>,
}

pub async fn device_info(pool: web::Data<DbPool>) -> Device {
    let auth = get_token_authoritzation(pool).await;
    
    let client = reqwest::Client::new();
    let res = client.get("https://api.spotify.com/v1/me/player")
        .header("Authorization", auth)
        .send()
        .await;

    let raw_response: Value = serde_json::from_str(&res.unwrap().text().await.unwrap()).unwrap();

    let is_active = match raw_response["is_playing"].to_string().as_str() {
        "true" => true,
        "false" => false,
        _ => false
    };

    println!("is active: {is_active}");

    if !is_active {
        return Device {
            id: raw_response["device"]["id"].to_string(),
            is_active: is_active,
            name: raw_response["device"]["name"].to_string(),
            song_name: None,
            song_artist: None,
        };
    }

    return Device {
        id: raw_response["device"]["id"].to_string(),
        is_active: is_active,
        name: raw_response["device"]["name"].to_string(),
        song_name: Some(raw_response["item"]["name"].to_string()),
        song_artist: Some(raw_response["item"]["artists"][0]["name"].to_string()),
    };
}