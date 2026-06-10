use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::VecDeque;

#[derive(Serialize, Clone, Default)]
pub struct SystemMetrics {
    pub service: String,
    pub os_info: String,
    pub cpu_cores: String,
    pub cpu_usage: String,
    pub memory_usage: String,
}

#[derive(Serialize)]
pub struct SpotifyStatus {
    pub is_playing: bool,
    pub is_recently_played: bool,
    pub title: String,
    pub artist: String,
    pub album_art: String,
    pub progress_ms: u64,
    pub duration_ms: u64,
    pub track_url: String,
}

#[derive(Deserialize)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
}

#[derive(Serialize, Clone, Default)]
pub struct TelemetryPoint {
    pub cpu: f32,
    pub memory: f32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NetworkTarget {
    pub target: String,
    pub name: String,
    pub latency_ms: f32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NetworkMetrics {
    pub google_dns: NetworkTarget,
    pub cloudflare_dns: NetworkTarget,
    pub riot_games: NetworkTarget,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NetworkHistoryPoint {
    pub google_dns: f32,
    pub cloudflare_dns: f32,
    pub riot_games: f32,
}

#[derive(Clone)]
pub struct AppState {
    pub metrics: Arc<RwLock<SystemMetrics>>,
    pub history: Arc<RwLock<VecDeque<TelemetryPoint>>>,
    pub network_metrics: Arc<RwLock<NetworkMetrics>>,
    pub network_history: Arc<RwLock<VecDeque<NetworkHistoryPoint>>>,
}
