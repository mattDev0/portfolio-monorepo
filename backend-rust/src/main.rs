use axum::{routing::get, Router, Json};
use serde::Serialize;
use sysinfo::System;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

// Updated struct to hold real system data
#[derive(Serialize)]
struct SystemMetrics {
    service: String,
    os_info: String,
    cpu_cores: String,
    memory_usage: String,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/api/status", get(get_system_status))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("🚀 Rust Engine live on http://localhost:8080");
    
    axum::serve(listener, app).await.unwrap();
}

// This function now queries the actual hardware/OS layer
async fn get_system_status() -> Json<SystemMetrics> {
    // Initialize the system reader
    let mut sys = System::new_all();
    sys.refresh_all(); // Fetch latest data

    // Calculate memory in Megabytes
    let total_mem_mb = sys.total_memory() / 1024 / 1024;
    let used_mem_mb = sys.used_memory() / 1024 / 1024;
    
    // Get OS Details
    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "".to_string());
    
    // Get CPU Cores
    let cpu_count = sys.cpus().len();

    let response = SystemMetrics {
        service: "Rust Hardware Monitor".to_string(),
        os_info: format!("{} {}", os_name, os_version),
        cpu_cores: format!("{} Logical Cores", cpu_count),
        memory_usage: format!("{} MB / {} MB", used_mem_mb, total_mem_mb),
    };

    Json(response)
}