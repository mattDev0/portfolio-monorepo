use sysinfo::System;
use tokio::time::{sleep, Duration};
use crate::models::{AppState, SystemMetrics, TelemetryPoint};
use tokio_util::sync::CancellationToken;

pub fn start_system_monitor(state: AppState, cancel_token: CancellationToken) -> tokio::task::JoinHandle<()> {
    let metrics_clone = state.metrics.clone();
    let history_clone = state.history.clone();

    tokio::spawn(async move {
        let mut sys = System::new_all();
        // Warm up the CPU readings
        sys.refresh_cpu_all();
        sleep(Duration::from_millis(200)).await;

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
                    cpu_core_count: cpu_count as u32,
                    cpu_usage_percent: cpu_usage,
                    memory_used_mb: used_mem_mb,
                    memory_total_mb: total_mem_mb,
                };
            }

            {
                let mut history_guard = history_clone.write().await;
                history_guard.push_back(TelemetryPoint {
                    cpu: cpu_usage,
                    memory: memory_pct,
                });
                if history_guard.len() > 20 {
                    history_guard.pop_front();
                }
            }

            tokio::select! {
                _ = cancel_token.cancelled() => {
                    tracing::info!("System monitor gracefully shutting down.");
                    break;
                }
                _ = sleep(Duration::from_secs(3)) => {}
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use std::collections::VecDeque;

    #[tokio::test]
    async fn test_start_system_monitor() {
        let metrics_state = AppState {
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
            history: Arc::new(RwLock::new(VecDeque::new())),
            network_metrics: Arc::new(RwLock::new(crate::models::NetworkMetrics::default())),
            network_history: Arc::new(RwLock::new(VecDeque::new())),
            spotify_cache: Arc::new(RwLock::new(None)),
        };

        let cancel_token = CancellationToken::new();
        start_system_monitor(metrics_state.clone(), cancel_token.clone());

        // Wait for the spawned monitor to run at least one iteration
        sleep(Duration::from_millis(600)).await;

        let guard = metrics_state.metrics.read().await;
        assert_eq!(guard.service, "Rust Hardware Monitor");
        assert!(guard.cpu_core_count > 0);
        assert!(!guard.os_info.is_empty());

        let history_guard = metrics_state.history.read().await;
        assert!(!history_guard.is_empty());
        assert!(history_guard.len() <= 20);
    }
}

