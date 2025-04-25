use actix_web::web;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use base64::prelude::*;
use reqwest;
use crate::auth::DbPool;

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

pub async fn add_song(uri: String, pool: web::Data<DbPool>) {
    let auth = auth::get_token_authoritzation(pool).await;

    let params = vec![
        ("uri", uri)
    ];

    let client = reqwest::Client::new();
    let res = client.post("https://api.spotify.com/v1/me/player/queue")
        .query(&params)
        .header("Authorization", auth.clone())
        .header("Content-Length", 0)
        .send()
        .await
        .unwrap();

    let res1 = client.get("https://api.spotify.com/v1/me/player")
        .header("Authorization", auth)
        .send()
        .await
        .unwrap();

    let device: serde_json::Value = serde_json::from_str(&res1.text().await.unwrap()).unwrap();
    
    println!("device code: {d}", d = device["id"]);
    println!("response came back with: {code}", code = res.status());
}