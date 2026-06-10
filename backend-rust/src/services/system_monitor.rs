use sysinfo::System;
use tokio::time::{sleep, Duration};
use crate::models::{AppState, SystemMetrics, TelemetryPoint};

pub fn start_system_monitor(state: AppState) {
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
                    cpu_cores: format!("{} Logical Cores", cpu_count),
                    cpu_usage: format!("{:.1}%", cpu_usage),
                    memory_usage: format!("{} MB / {} MB", used_mem_mb, total_mem_mb),
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

            sleep(Duration::from_secs(3)).await;
        }
    });
}
