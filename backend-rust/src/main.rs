use axum::{routing::get, Router, Json};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

// We define a struct and tell Serde to automatically convert it to JSON
#[derive(Serialize)]
struct SystemStatus {
    service: String,
    status: String,
    uptime_concept: String,
}

#[tokio::main]
async fn main() {
    // 1. Configure CORS so our React UI on port 5173 can access this API
    let cors = CorsLayer::new().allow_origin(Any);

    // 2. Build our application router
    let app = Router::new()
        .route("/api/status", get(get_system_status))
        .layer(cors);

    // 3. Bind the server to a local port
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Rust microservice running on http://localhost:8080");
    
    // 4. Start the server
    axum::serve(listener, app).await.unwrap();
}

// 5. This is the handler function that gets called when React hits /api/status
async fn get_system_status() -> Json<SystemStatus> {
    let response = SystemStatus {
        service: "Rust Core Systems".to_string(),
        status: "Operational".to_string(),
        uptime_concept: "Zero-cost abstractions active".to_string(),
    };

    Json(response)
}