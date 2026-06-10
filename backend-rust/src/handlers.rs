use axum::{extract::State, Json};
use crate::models::{AppState, SystemMetrics, TelemetryPoint, NetworkMetrics, NetworkHistoryPoint, SpotifyStatus};
use crate::services::spotify_client;

pub async fn get_system_status(State(state): State<AppState>) -> Json<SystemMetrics> {
    let guard = state.metrics.read().await;
    Json(guard.clone())
}

pub async fn get_system_history(State(state): State<AppState>) -> Json<std::collections::VecDeque<TelemetryPoint>> {
    let guard = state.history.read().await;
    Json(guard.clone())
}

pub async fn get_network_status(State(state): State<AppState>) -> Json<NetworkMetrics> {
    let guard = state.network_metrics.read().await;
    Json(guard.clone())
}

pub async fn get_network_history(State(state): State<AppState>) -> Json<std::collections::VecDeque<NetworkHistoryPoint>> {
    let guard = state.network_history.read().await;
    Json(guard.clone())
}

pub async fn get_spotify(State(_state): State<AppState>) -> Json<SpotifyStatus> {
    spotify_client::get_spotify_status().await
}
