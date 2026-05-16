use axum::{routing::get, Router, Json};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use sysinfo::System;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

// Existing Hardware Struct
#[derive(Serialize)]
struct SystemMetrics {
    service: String,
    os_info: String,
    cpu_cores: String,
    memory_usage: String,
}

// New Spotify Structs
#[derive(Serialize)]
struct SpotifyStatus {
    is_playing: bool,
    title: String,
    artist: String,
    album_art: String,
}

#[derive(Deserialize)]
struct SpotifyTokenResponse {
    access_token: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/status", get(get_system_status))
        .route("/api/spotify", get(get_spotify_status)) // New Route
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Rust Engine live on http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_system_status() -> Json<SystemMetrics> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let total_mem_mb = sys.total_memory() / 1024 / 1024;
    let used_mem_mb = sys.used_memory() / 1024 / 1024;
    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let cpu_count = sys.cpus().len();

    Json(SystemMetrics {
        service: "Rust Hardware Monitor".to_string(),
        os_info: format!("{} ", os_name),
        cpu_cores: format!("{} Logical Cores", cpu_count),
        memory_usage: format!("{} MB / {} MB", used_mem_mb, total_mem_mb),
    })
}

// The core Spotify logic
async fn get_spotify_status() -> Json<SpotifyStatus> {
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("Missing CLIENT_ID");
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").expect("Missing CLIENT_SECRET");
    let refresh_token = env::var("SPOTIFY_REFRESH_TOKEN").expect("Missing REFRESH_TOKEN");

    let client = Client::new();

    // 1. Trade the refresh token for a live access token
    // 1. Trade the refresh token for a live access token
    let auth_header = format!(
        "Basic {}",
        base64::encode(format!("{}:{}", client_id, client_secret))
    );

    let token_request = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", auth_header)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
        ])
        .send()
        .await
        .unwrap();

    // Extract the raw text from Spotify before trying to decode it
    let raw_response = token_request.text().await.unwrap();

    // Try to parse the JSON gracefully
    let token_res: SpotifyTokenResponse = match serde_json::from_str(&raw_response) {
        Ok(parsed) => parsed,
        Err(_) => {
            // If it fails, print Spotify's exact error to the Linux terminal!
            println!("❌ SPOTIFY AUTH ERROR: {}", raw_response);
            
            // Return a safe "Offline" state to React so the UI doesn't crash
            return Json(SpotifyStatus {
                is_playing: false,
                title: "Auth Error".to_string(),
                artist: "Check Rust Terminal".to_string(),
                album_art: "".to_string(),
            });
        }
    };

    // 2. Query the "Currently Playing" endpoint
    let playing_res = client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .header("Authorization", format!("Bearer {}", token_res.access_token))
        .send()
        .await
        .unwrap();

    // 3. Parse the data. If nothing is playing, return a default state.
    if playing_res.status() == 204 || playing_res.status() == 202 {
         return Json(SpotifyStatus {
             is_playing: false,
             title: "Offline".to_string(),
             artist: "No active session".to_string(),
             album_art: "".to_string(),
         });
    }

    let track_data: serde_json::Value = playing_res.json().await.unwrap();
    
    // Safely extract JSON fields
    let is_playing = track_data["is_playing"].as_bool().unwrap_or(false);
    let title = track_data["item"]["name"].as_str().unwrap_or("Unknown").to_string();
    let artist = track_data["item"]["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string();
    let album_art = track_data["item"]["album"]["images"][0]["url"].as_str().unwrap_or("").to_string();

    Json(SpotifyStatus {
        is_playing,
        title,
        artist,
        album_art,
    })
}