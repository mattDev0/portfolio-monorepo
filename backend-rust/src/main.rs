mod handlers;
mod models;
mod services;
mod utils;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use handlers::*;
use models::{AppState, NetworkMetrics, SystemMetrics};
use services::{network_monitor, system_monitor};
use tokio_util::sync::CancellationToken;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

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
        spotify_cache: Arc::new(RwLock::new(None)),
        spotify_token: Arc::new(RwLock::new(None)),
        spotify_fresh: Arc::new(RwLock::new(None)),
    };

    // 2. Spawn background tasks to refresh telemetry periodically
    let cancel_token = CancellationToken::new();
    let sys_handle =
        system_monitor::start_system_monitor(metrics_state.clone(), cancel_token.clone());
    let net_handle =
        network_monitor::start_network_monitor(metrics_state.clone(), cancel_token.clone());

    let app = build_app(metrics_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("🚀 Rust Engine live on http://localhost:8080");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(cancel_token.clone()))
    .await?;

    let _ = tokio::join!(sys_handle, net_handle);

    Ok(())
}

fn build_app(state: AppState) -> Router {
    let allowed_origin_str = std::env::var("CORS_ALLOWED_ORIGIN").unwrap_or_else(|_| {
        if utils::is_local_runtime() {
            "http://localhost:5173".to_string()
        } else {
            "https://mattdev0.tech".to_string()
        }
    });

    let allowed_origins: Vec<axum::http::HeaderValue> = allowed_origin_str
        .split(',')
        .map(|s| s.trim().parse().expect("Invalid CORS origin"))
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(Any)
        .allow_headers(Any);

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // Rate limiter: 60 requests per minute per IP, applied ONLY to /api/* routes.
    // Uses SmartIpKeyExtractor to read client IP from proxy headers (e.g. X-Forwarded-For).
    // The rate-limit layer is applied only to the /api/* sub-router, so /health and
    // /healthz are never rate-limited.
    let mut builder = tower_governor::governor::GovernorConfigBuilder::default()
        .key_extractor(tower_governor::key_extractor::SmartIpKeyExtractor);
    builder.per_second(1);
    builder.burst_size(60);
    let governor_conf = Arc::new(builder.finish().expect("Failed to create governor config"));
    let rate_limit_layer = tower_governor::GovernorLayer::new(governor_conf);

    // API routes — rate-limited
    let api_routes = Router::new()
        .route("/api/status", get(get_system_status))
        .route("/api/status/history", get(get_system_history))
        .route("/api/status/network", get(get_network_status))
        .route("/api/status/network/history", get(get_network_history))
        .route("/api/spotify", get(get_spotify))
        .layer(rate_limit_layer);

    // Health routes — NOT rate-limited.
    // External requests to /healthz through the Cloud Run URL are intercepted
    // by the GFE, so /health is the externally reachable health endpoint, and
    // /healthz stays for the Kubernetes probes in infrastructure/k8s/rust-api.yaml.
    Router::new()
        .route("/health", get(get_health))
        .route("/healthz", get(get_health))
        .merge(api_routes)
        .layer(cors)
        .layer(trace_layer)
        .with_state(state)
}

async fn shutdown_signal(cancel_token: CancellationToken) {
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
    cancel_token.cancel();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use std::collections::VecDeque;
    use tower::ServiceExt;

    fn test_state() -> AppState {
        AppState {
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
            history: Arc::new(RwLock::new(VecDeque::new())),
            network_metrics: Arc::new(RwLock::new(NetworkMetrics::default())),
            network_history: Arc::new(RwLock::new(VecDeque::new())),
            spotify_cache: Arc::new(RwLock::new(None)),
            spotify_token: Arc::new(RwLock::new(None)),
            spotify_fresh: Arc::new(RwLock::new(None)),
        }
    }

    #[tokio::test]
    async fn health_and_healthz_return_ok_and_match() {
        let app = build_app(test_state());

        let req_health = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let res_health = app.clone().oneshot(req_health).await.unwrap();
        assert_eq!(res_health.status(), StatusCode::OK);
        let body_health = axum::body::to_bytes(res_health.into_body(), usize::MAX)
            .await
            .unwrap();

        let req_healthz = Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .unwrap();
        let res_healthz = app.oneshot(req_healthz).await.unwrap();
        assert_eq!(res_healthz.status(), StatusCode::OK);
        let body_healthz = axum::body::to_bytes(res_healthz.into_body(), usize::MAX)
            .await
            .unwrap();

        assert_eq!(body_health, body_healthz);
        let json: serde_json::Value = serde_json::from_slice(&body_health).unwrap();
        assert_eq!(json, serde_json::json!({ "status": "ok" }));
    }

    #[tokio::test]
    async fn health_route_is_not_rate_limited() {
        let app = build_app(test_state());

        for _ in 0..100 {
            let req = Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap();
            let res = app.clone().oneshot(req).await.unwrap();
            assert_eq!(res.status(), StatusCode::OK);
        }
    }
}
