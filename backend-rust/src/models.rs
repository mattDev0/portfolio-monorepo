use serde::Deserialize;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use portfolio_api_types::{
    NetworkHistoryPoint, NetworkMetrics, NetworkTarget, SpotifyStatus, SystemMetrics,
    TelemetryPoint,
};

#[derive(Deserialize)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub expires_in: u64,
}

// An access token held in memory so we stop trading the refresh token on
// every single request. `scope` is kept so callers can skip endpoints the
// token was never granted access to.
#[derive(Clone)]
pub struct CachedToken {
    pub access_token: String,
    pub scope: String,
    pub expires_at: std::time::Instant,
}

#[derive(Clone)]
pub struct AppState {
    pub metrics: Arc<RwLock<SystemMetrics>>,
    pub history: Arc<RwLock<VecDeque<TelemetryPoint>>>,
    pub network_metrics: Arc<RwLock<NetworkMetrics>>,
    pub network_history: Arc<RwLock<VecDeque<NetworkHistoryPoint>>>,
    pub spotify_cache: Arc<RwLock<Option<SpotifyStatus>>>,
    pub spotify_token: Arc<RwLock<Option<CachedToken>>>,
    pub spotify_fresh: Arc<RwLock<Option<(SpotifyStatus, std::time::Instant)>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_types_serialization_snapshot() {
        let system_metrics = SystemMetrics {
            service: "backend-rust".to_string(),
            os_info: "Linux 6.6.137 x86_64".to_string(),
            cpu_core_count: 8,
            cpu_usage_percent: 24.5,
            memory_used_mb: 512,
            memory_total_mb: 2048,
        };

        let spotify_status = SpotifyStatus {
            is_playing: true,
            is_recently_played: false,
            title: "Time".to_string(),
            artist: "Hans Zimmer".to_string(),
            album_art: "https://example.com/art.jpg".to_string(),
            progress_ms: 120000,
            duration_ms: 275000,
            track_url: "https://open.spotify.com/track/12345".to_string(),
        };

        let telemetry_point = TelemetryPoint {
            cpu: 18.25,
            memory: 45.5,
        };

        let google_dns = NetworkTarget {
            target: "8.8.8.8".to_string(),
            name: "Google DNS".to_string(),
            latency_ms: 14.5,
            status: "online".to_string(),
        };

        let cloudflare_dns = NetworkTarget {
            target: "1.1.1.1".to_string(),
            name: "Cloudflare DNS".to_string(),
            latency_ms: 9.75,
            status: "online".to_string(),
        };

        let riot_games = NetworkTarget {
            target: "104.160.131.1".to_string(),
            name: "Riot Games".to_string(),
            latency_ms: 32.0,
            status: "degraded".to_string(),
        };

        let network_metrics = NetworkMetrics {
            google_dns: google_dns.clone(),
            cloudflare_dns: cloudflare_dns.clone(),
            riot_games: riot_games.clone(),
        };

        let network_history_point = NetworkHistoryPoint {
            google_dns: 14.5,
            cloudflare_dns: 9.75,
            riot_games: 32.0,
        };

        assert_eq!(
            serde_json::to_value(&system_metrics).unwrap()["service"],
            "backend-rust"
        );
        assert_eq!(
            serde_json::to_value(&spotify_status).unwrap()["title"],
            "Time"
        );
        assert_eq!(
            serde_json::to_value(&telemetry_point).unwrap()["cpu"],
            18.25
        );
        assert_eq!(
            serde_json::to_value(&google_dns).unwrap()["name"],
            "Google DNS"
        );
        assert_eq!(
            serde_json::to_value(&network_metrics).unwrap()["cloudflare_dns"]["name"],
            "Cloudflare DNS"
        );
        assert_eq!(
            serde_json::to_value(&network_history_point).unwrap()["riot_games"],
            32.0
        );
    }
}
