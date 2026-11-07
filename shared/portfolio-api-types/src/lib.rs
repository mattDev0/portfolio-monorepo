use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct SystemMetrics {
    pub service: String,
    pub os_info: String,
    pub cpu_core_count: u32,
    pub cpu_usage_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct TelemetryPoint {
    pub cpu: f32,
    pub memory: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct NetworkTarget {
    pub target: String,
    pub name: String,
    pub latency_ms: f32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct NetworkMetrics {
    pub google_dns: NetworkTarget,
    pub cloudflare_dns: NetworkTarget,
    pub riot_games: NetworkTarget,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct NetworkHistoryPoint {
    pub google_dns: f32,
    pub cloudflare_dns: f32,
    pub riot_games: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct GitHubCommitActivity {
    pub repo: String,
    pub message: String,
    pub date: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(default)]
pub struct InfrastructureMetrics {
    pub engine: String,
    pub uptime_hours: i64,
    pub uptime_minutes: i64,
    pub jvm_memory_used_mb: i64,
    pub jvm_memory_total_mb: i64,
    pub active_threads: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_system_metrics_roundtrip() {
        let sample = json!({
            "service": "backend-rust",
            "os_info": "Linux 6.6.137",
            "cpu_core_count": 4,
            "cpu_usage_percent": 12.5,
            "memory_used_mb": 256,
            "memory_total_mb": 1024
        });
        let parsed: SystemMetrics = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_spotify_status_roundtrip() {
        let sample = json!({
            "is_playing": true,
            "is_recently_played": false,
            "title": "Song Title",
            "artist": "Artist Name",
            "album_art": "https://example.com/art.jpg",
            "progress_ms": 30000,
            "duration_ms": 180000,
            "track_url": "https://open.spotify.com/track/123"
        });
        let parsed: SpotifyStatus = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_telemetry_point_roundtrip() {
        let sample = json!({
            "cpu": 25.5,
            "memory": 40.0
        });
        let parsed: TelemetryPoint = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_network_target_roundtrip() {
        let sample = json!({
            "target": "8.8.8.8",
            "name": "Google DNS",
            "latency_ms": 14.25,
            "status": "online"
        });
        let parsed: NetworkTarget = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_network_metrics_roundtrip() {
        let sample = json!({
            "google_dns": {
                "target": "8.8.8.8",
                "name": "Google DNS",
                "latency_ms": 14.25,
                "status": "online"
            },
            "cloudflare_dns": {
                "target": "1.1.1.1",
                "name": "Cloudflare DNS",
                "latency_ms": 11.75,
                "status": "online"
            },
            "riot_games": {
                "target": "104.160.131.3",
                "name": "Riot Games NA",
                "latency_ms": 42.5,
                "status": "online"
            }
        });
        let parsed: NetworkMetrics = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_network_history_point_roundtrip() {
        let sample = json!({
            "google_dns": 14.25,
            "cloudflare_dns": 11.75,
            "riot_games": 42.5
        });
        let parsed: NetworkHistoryPoint = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_github_commit_activity_roundtrip() {
        let sample = json!({
            "repo": "portfolio-monorepo",
            "message": "feat: initial commit",
            "date": "2026-11-06T12:00:00Z",
            "hash": "abc1234"
        });
        let parsed: GitHubCommitActivity = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_infrastructure_metrics_roundtrip() {
        let sample = json!({
            "engine": "Spring Boot 4.0.0",
            "uptime_hours": 12,
            "uptime_minutes": 34,
            "jvm_memory_used_mb": 128,
            "jvm_memory_total_mb": 512,
            "active_threads": 8
        });
        let parsed: InfrastructureMetrics = serde_json::from_value(sample.clone()).expect("parse");
        let serialized = serde_json::to_value(&parsed).expect("serialize");
        assert_eq!(serialized, sample);
    }

    #[test]
    fn test_missing_field_uses_default() {
        let sample = json!({
            "service": "backend-rust",
            "cpu_usage_percent": 15.0
        });
        let parsed: SystemMetrics = serde_json::from_value(sample).expect("parse partial");
        assert_eq!(parsed.service, "backend-rust");
        assert_eq!(parsed.cpu_usage_percent, 15.0);
        assert_eq!(parsed.os_info, "");
        assert_eq!(parsed.cpu_core_count, 0);
        assert_eq!(parsed.memory_used_mb, 0);
        assert_eq!(parsed.memory_total_mb, 0);
    }
}
