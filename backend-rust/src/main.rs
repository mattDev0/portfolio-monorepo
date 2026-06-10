mod models;
mod utils;
mod services;
mod handlers;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use models::{AppState, SystemMetrics, NetworkMetrics};
use services::{system_monitor, network_monitor};
use handlers::*;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    // 1. Initialize thread-safe shared state for hardware and network telemetry
    let metrics_state = AppState {
        metrics: Arc::new(RwLock::new(SystemMetrics::default())),
        history: Arc::new(RwLock::new(std::collections::VecDeque::new())),
        network_metrics: Arc::new(RwLock::new(NetworkMetrics::default())),
        network_history: Arc::new(RwLock::new(std::collections::VecDeque::new())),
    };

    // 2. Spawn background tasks to refresh telemetry periodically
    system_monitor::start_system_monitor(metrics_state.clone());
    network_monitor::start_network_monitor(metrics_state.clone());

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/status", get(get_system_status))
        .route("/api/status/history", get(get_system_history))
        .route("/api/status/network", get(get_network_status))
        .route("/api/status/network/history", get(get_network_history))
        .route("/api/spotify", get(get_spotify))
        .layer(cors)
        .with_state(metrics_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Rust Engine live on http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}
