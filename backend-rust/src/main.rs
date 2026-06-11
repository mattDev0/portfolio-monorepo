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
use tower_http::trace::TraceLayer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    // Initialize tracing subscriber to emit JSON logs
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .json()
        .init();


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

    let allowed_origin_str = std::env::var("CORS_ALLOWED_ORIGIN")
        .unwrap_or_else(|_| {
            if utils::is_local_runtime() {
                "http://localhost:5173".to_string()
            } else {
                "https://mattdev0.tech".to_string()
            }
        });

    let allowed_origin = allowed_origin_str
        .parse::<axum::http::HeaderValue>()
        .expect("Invalid CORS_ALLOWED_ORIGIN");

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/healthz", get(get_health))
        .route("/api/status", get(get_system_status))
        .route("/api/status/history", get(get_system_history))
        .route("/api/status/network", get(get_network_status))
        .route("/api/status/network/history", get(get_network_history))
        .route("/api/spotify", get(get_spotify))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(metrics_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("🚀 Rust Engine live on http://localhost:8080");
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Graceful shutdown initiated...");
}
