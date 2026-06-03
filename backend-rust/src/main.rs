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
use base64::Engine;

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
    is_recently_played: bool,
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

#[derive(Serialize, Clone, Default)]
struct TelemetryPoint {
    cpu: f32,
    memory: f32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NetworkTarget {
    target: String,
    name: String,
    latency_ms: f32,
    status: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NetworkMetrics {
    google_dns: NetworkTarget,
    cloudflare_dns: NetworkTarget,
    riot_games: NetworkTarget,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NetworkHistoryPoint {
    google_dns: f32,
    cloudflare_dns: f32,
    riot_games: f32,
}

// AppState to hold the shared system metrics and network metrics
#[derive(Clone)]
struct AppState {
    metrics: Arc<RwLock<SystemMetrics>>,
    history: Arc<RwLock<Vec<TelemetryPoint>>>,
    network_metrics: Arc<RwLock<NetworkMetrics>>,
    network_history: Arc<RwLock<Vec<NetworkHistoryPoint>>>,
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    // 1. Initialize thread-safe shared state for hardware and network telemetry
    let metrics_state = AppState {
        metrics: Arc::new(RwLock::new(SystemMetrics::default())),
        history: Arc::new(RwLock::new(Vec::new())),
        network_metrics: Arc::new(RwLock::new(NetworkMetrics::default())),
        network_history: Arc::new(RwLock::new(Vec::new())),
    };

    // 2. Spawn a background task to refresh hardware telemetry periodically
    let metrics_clone = metrics_state.metrics.clone();
    let history_clone = metrics_state.history.clone();
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
            let memory_pct = if sys.total_memory() > 0 {
                (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as f32
            } else {
                0.0
            };

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

            {
                let mut history_guard = history_clone.write().await;
                history_guard.push(TelemetryPoint {
                    cpu: cpu_usage,
                    memory: memory_pct,
                });
                if history_guard.len() > 20 {
                    history_guard.remove(0);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    });

    // 3. Spawn a background task to refresh network latency telemetry periodically
    let net_metrics_clone = metrics_state.network_metrics.clone();
    let net_history_clone = metrics_state.network_history.clone();
    tokio::spawn(async move {
        let client = Client::new();
        let prometheus_url = env::var("PROMETHEUS_URL")
            .unwrap_or_else(|_| "http://localhost:9090".to_string());

        loop {
            let net_metrics = match fetch_network_metrics(&client, &prometheus_url).await {
                Some(m) => m,
                None => generate_mock_network_metrics(),
            };

            {
                let mut guard = net_metrics_clone.write().await;
                *guard = net_metrics.clone();
            }

            {
                let mut history_guard = net_history_clone.write().await;
                history_guard.push(NetworkHistoryPoint {
                    google_dns: net_metrics.google_dns.latency_ms,
                    cloudflare_dns: net_metrics.cloudflare_dns.latency_ms,
                    riot_games: net_metrics.riot_games.latency_ms,
                });
                if history_guard.len() > 20 {
                    history_guard.remove(0);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/status", get(get_system_status))
        .route("/api/status/history", get(get_system_history))
        .route("/api/status/network", get(get_network_status))
        .route("/api/status/network/history", get(get_network_history))
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

async fn get_system_history(State(state): State<AppState>) -> Json<Vec<TelemetryPoint>> {
    let guard = state.history.read().await;
    Json(guard.clone())
}

async fn get_network_status(State(state): State<AppState>) -> Json<NetworkMetrics> {
    let guard = state.network_metrics.read().await;
    Json(guard.clone())
}

async fn get_network_history(State(state): State<AppState>) -> Json<Vec<NetworkHistoryPoint>> {
    let guard = state.network_history.read().await;
    Json(guard.clone())
}

fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

async fn query_prometheus_metric(client: &Client, prometheus_url: &str, query: &str) -> Option<serde_json::Value> {
    let url = format!("{}/api/v1/query?query={}", prometheus_url, url_encode(query));
    let res = client
        .get(&url)
        .send()
        .await
        .ok()?;
    
    if res.status() == 200 {
        res.json::<serde_json::Value>().await.ok()
    } else {
        None
    }
}

fn get_metric_value_for_instance(json: &serde_json::Value, instance: &str) -> Option<f32> {
    let results = json.get("data")?.get("result")?.as_array()?;
    for result in results {
        let metric_instance = result.get("metric")?.get("instance")?.as_str()?;
        if metric_instance == instance {
            let value_arr = result.get("value")?.as_array()?;
            let value_str = value_arr.get(1)?.as_str()?;
            return value_str.parse::<f32>().ok();
        }
    }
    None
}

async fn fetch_network_metrics(client: &Client, prometheus_url: &str) -> Option<NetworkMetrics> {
    let duration_json = query_prometheus_metric(client, prometheus_url, "probe_duration_seconds{job=\"network_latency_ping\"}").await?;
    let success_json = query_prometheus_metric(client, prometheus_url, "probe_success{job=\"network_latency_ping\"}").await?;

    let google_lat = get_metric_value_for_instance(&duration_json, "8.8.8.8")? * 1000.0;
    let google_suc = get_metric_value_for_instance(&success_json, "8.8.8.8")?.round() as i32;

    let cloudflare_lat = get_metric_value_for_instance(&duration_json, "1.1.1.1")? * 1000.0;
    let cloudflare_suc = get_metric_value_for_instance(&success_json, "1.1.1.1")?.round() as i32;

    let riot_lat = get_metric_value_for_instance(&duration_json, "104.160.131.3")? * 1000.0;
    let riot_suc = get_metric_value_for_instance(&success_json, "104.160.131.3")?.round() as i32;

    Some(NetworkMetrics {
        google_dns: NetworkTarget {
            target: "8.8.8.8".to_string(),
            name: "Google DNS".to_string(),
            latency_ms: (google_lat * 10.0).round() / 10.0,
            status: if google_suc == 1 { "online".to_string() } else { "offline".to_string() },
        },
        cloudflare_dns: NetworkTarget {
            target: "1.1.1.1".to_string(),
            name: "Cloudflare DNS".to_string(),
            latency_ms: (cloudflare_lat * 10.0).round() / 10.0,
            status: if cloudflare_suc == 1 { "online".to_string() } else { "offline".to_string() },
        },
        riot_games: NetworkTarget {
            target: "104.160.131.3".to_string(),
            name: "Riot Games NA".to_string(),
            latency_ms: (riot_lat * 10.0).round() / 10.0,
            status: if riot_suc == 1 { "online".to_string() } else { "offline".to_string() },
        },
    })
}

fn generate_mock_network_metrics() -> NetworkMetrics {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let osc1 = (((now % 10000) as f32 / 10000.0) * 2.0 * std::f32::consts::PI).sin();
    let osc2 = (((now % 7000) as f32 / 7000.0) * 2.0 * std::f32::consts::PI).cos();
    
    let google_latency = 12.0 + osc1 * 1.8;
    let cloudflare_latency = 8.5 + osc2 * 1.2;
    let riot_latency = 39.0 + osc1 * 3.5;

    NetworkMetrics {
        google_dns: NetworkTarget {
            target: "8.8.8.8".to_string(),
            name: "Google DNS".to_string(),
            latency_ms: (google_latency * 10.0).round() / 10.0,
            status: "online".to_string(),
        },
        cloudflare_dns: NetworkTarget {
            target: "1.1.1.1".to_string(),
            name: "Cloudflare DNS".to_string(),
            latency_ms: (cloudflare_latency * 10.0).round() / 10.0,
            status: "online".to_string(),
        },
        riot_games: NetworkTarget {
            target: "104.160.131.3".to_string(),
            name: "Riot Games NA".to_string(),
            latency_ms: (riot_latency * 10.0).round() / 10.0,
            status: "online".to_string(),
        },
    }
}

// Helper for offline fallback status
fn spotify_offline_fallback(reason: &str) -> SpotifyStatus {
    SpotifyStatus {
        is_playing: false,
        is_recently_played: false,
        title: "Offline".to_string(),
        artist: reason.to_string(),
        album_art: "".to_string(),
        progress_ms: 0,
        duration_ms: 0,
        track_url: "".to_string(),
    }
}

// Helper to query recently played track
async fn get_recently_played(client: &Client, access_token: &str) -> Json<SpotifyStatus> {
    let recently_played_res = client
        .get("https://api.spotify.com/v1/me/player/recently-played?limit=1")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;

    if let Ok(res) = recently_played_res {
        if res.status() == 200 {
            if let Ok(recent_data) = res.json::<serde_json::Value>().await {
                if let Some(item) = recent_data["items"].get(0) {
                    let track = &item["track"];
                    let title = track["name"].as_str().unwrap_or("Unknown").to_string();
                    let artist = track["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string();
                    let album_art = track["album"]["images"][0]["url"].as_str().unwrap_or("").to_string();
                    let track_url = track["external_urls"]["spotify"].as_str().unwrap_or("").to_string();
                    let duration_ms = track["duration_ms"].as_u64().unwrap_or(0);

                    return Json(SpotifyStatus {
                        is_playing: false,
                        is_recently_played: true,
                        title,
                        artist,
                        album_art,
                        progress_ms: 0,
                        duration_ms,
                        track_url,
                    });
                }
            }
        }
    }

    Json(spotify_offline_fallback("No active session"))
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
                is_recently_played: false,
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
                is_recently_played: false,
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
        base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret))
    );

    let token_request = match client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", auth_header)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
        ])
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("❌ SPOTIFY REQ ERROR: {}", e);
            return Json(spotify_offline_fallback("Network Error"));
        }
    };

    // Extract the raw text from Spotify before trying to decode it
    let raw_response = match token_request.text().await {
        Ok(txt) => txt,
        Err(e) => {
            println!("❌ SPOTIFY TEXT READ ERROR: {}", e);
            return Json(spotify_offline_fallback("Text Read Error"));
        }
    };

    // Try to parse the JSON gracefully
    let token_res: SpotifyTokenResponse = match serde_json::from_str(&raw_response) {
        Ok(parsed) => parsed,
        Err(_) => {
            // If it fails, print Spotify's exact error to the Linux terminal!
            println!("❌ SPOTIFY AUTH ERROR: {}", raw_response);
            
            // Return a safe "Offline" state to React so the UI doesn't crash
            return Json(SpotifyStatus {
                is_playing: false,
                is_recently_played: false,
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
    let playing_res = match client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .header("Authorization", format!("Bearer {}", token_res.access_token))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("❌ SPOTIFY CURRENT REQ ERROR: {}", e);
            return Json(spotify_offline_fallback("API Error"));
        }
    };

    // 3. Parse the data. If nothing is playing, check recently played.
    if playing_res.status() == 204 || playing_res.status() == 202 {
        return get_recently_played(&client, &token_res.access_token).await;
    }

    let track_data: serde_json::Value = match playing_res.json().await {
        Ok(json) => json,
        Err(e) => {
            println!("❌ SPOTIFY JSON PARSE ERROR: {}", e);
            return get_recently_played(&client, &token_res.access_token).await;
        }
    };
    
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
        is_recently_played: !is_playing,
        title,
        artist,
        album_art,
        progress_ms,
        duration_ms,
        track_url,
    })
}
