use axum::{routing::get, Router, Json, extract::State};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use sysinfo::System;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

// Cache speed test change
fn is_local_runtime() -> bool {
    let app_env = env::var("APP_ENV")
        .or_else(|_| env::var("RUST_ENV"))
        .unwrap_or_default()
        .to_lowercase();

    matches!(app_env.as_str(), "local" | "development" | "dev")
        || (cfg!(debug_assertions) && app_env != "production")
}

// SystemMetrics struct now supports cpu_usage and Clone/Default
#[derive(Serialize, Clone, Default)]
struct SystemMetrics {
    service: String,
    os_info: String,
    cpu_cores: String,
    cpu_usage: String,
    memory_usage: String,
}

// New Spotify Structs with track_url
#[derive(Serialize)]
struct SpotifyStatus {
    is_playing: bool,
    title: String,
    artist: String,
    album_art: String,
    progress_ms: u64,
    duration_ms: u64,
    track_url: String,
}

#[derive(Deserialize)]
struct SpotifyTokenResponse {
    access_token: String,
}

// AppState to hold the shared system metrics
#[derive(Clone)]
struct AppState {
    metrics: Arc<RwLock<SystemMetrics>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    // 1. Initialize thread-safe shared state for hardware telemetry
    let metrics_state = AppState {
        metrics: Arc::new(RwLock::new(SystemMetrics::default())),
    };

    // 2. Spawn a background task to refresh telemetry periodically
    let metrics_clone = metrics_state.metrics.clone();
    tokio::spawn(async move {
        let mut sys = System::new_all();
        // Warm up the CPU readings
        sys.refresh_cpu_all();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        loop {
            sys.refresh_cpu_all();
            sys.refresh_memory();

            let total_mem_mb = sys.total_memory() / 1024 / 1024;
            let used_mem_mb = sys.used_memory() / 1024 / 1024;
            let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
            let cpu_count = sys.cpus().len();
            let cpu_usage = sys.global_cpu_usage();

            {
                let mut guard = metrics_clone.write().await;
                *guard = SystemMetrics {
                    service: "Rust Hardware Monitor".to_string(),
                    os_info: os_name,
                    cpu_cores: format!("{} Logical Cores", cpu_count),
                    cpu_usage: format!("{:.1}%", cpu_usage),
                    memory_usage: format!("{} MB / {} MB", used_mem_mb, total_mem_mb),
                };
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/status", get(get_system_status))
        .route("/api/spotify", get(get_spotify_status))
        .layer(cors)
        .with_state(metrics_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Rust Engine live on http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_system_status(State(state): State<AppState>) -> Json<SystemMetrics> {
    let guard = state.metrics.read().await;
    Json(guard.clone())
}

// The core Spotify logic
async fn get_spotify_status() -> Json<SpotifyStatus> {
    let spotify_credentials = (
        env::var("SPOTIFY_CLIENT_ID"),
        env::var("SPOTIFY_CLIENT_SECRET"),
        env::var("SPOTIFY_REFRESH_TOKEN"),
    );

    let (client_id, client_secret, refresh_token) = match spotify_credentials {
        (Ok(client_id), Ok(client_secret), Ok(refresh_token)) => {
            (client_id, client_secret, refresh_token)
        }
        _ if is_local_runtime() => {
            return Json(SpotifyStatus {
                is_playing: false,
                title: "Local Mode".to_string(),
                artist: "Spotify credentials not configured".to_string(),
                album_art: "".to_string(),
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
            });
        }
        _ => {
            println!("Missing Spotify credentials");
            return Json(SpotifyStatus {
                is_playing: false,
                title: "Offline".to_string(),
                artist: "Spotify credentials unavailable".to_string(),
                album_art: "".to_string(),
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
            });
        }
    };

    let client = Client::new();

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
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
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
             progress_ms: 0,
             duration_ms: 0,
             track_url: "".to_string(),
         });
    }

    let track_data: serde_json::Value = playing_res.json().await.unwrap();
    
    // Safely extract JSON fields
    let is_playing = track_data["is_playing"].as_bool().unwrap_or(false);
    let title = track_data["item"]["name"].as_str().unwrap_or("Unknown").to_string();
    let artist = track_data["item"]["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string();
    let album_art = track_data["item"]["album"]["images"][0]["url"].as_str().unwrap_or("").to_string();
    let track_url = track_data["item"]["external_urls"]["spotify"].as_str().unwrap_or("").to_string();
    
    let progress_ms = track_data["progress_ms"].as_u64().unwrap_or(0);
    let duration_ms = track_data["item"]["duration_ms"].as_u64().unwrap_or(0);

    Json(SpotifyStatus {
        is_playing,
        title,
        artist,
        album_art,
        progress_ms,
        duration_ms,
        track_url,
    })
}
