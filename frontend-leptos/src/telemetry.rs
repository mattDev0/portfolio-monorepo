use leptos::prelude::*;
use portfolio_api_types::{
    InfrastructureMetrics, NetworkHistoryPoint, NetworkMetrics, SystemMetrics, TelemetryPoint,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;

pub const MAX_TELEMETRY_HISTORY: usize = 20;

pub fn calculate_memory_percent(used_mb: u64, total_mb: u64) -> f32 {
    if total_mb == 0 {
        0.0
    } else {
        (used_mb as f32 / total_mb as f32) * 100.0
    }
}

pub fn append_telemetry_point(history: &mut Vec<TelemetryPoint>, cpu: f32, memory: f32) -> bool {
    if let Some(last) = history.last() {
        if (last.cpu - cpu).abs() < f32::EPSILON && (last.memory - memory).abs() < f32::EPSILON {
            return false;
        }
    }
    history.push(TelemetryPoint { cpu, memory });
    if history.len() > MAX_TELEMETRY_HISTORY {
        history.remove(0);
    }
    true
}

pub fn append_network_history_point(
    history: &mut Vec<NetworkHistoryPoint>,
    google: f32,
    cloudflare: f32,
    riot: f32,
) -> bool {
    if let Some(last) = history.last() {
        if (last.google_dns - google).abs() < f32::EPSILON
            && (last.cloudflare_dns - cloudflare).abs() < f32::EPSILON
            && (last.riot_games - riot).abs() < f32::EPSILON
        {
            return false;
        }
    }
    history.push(NetworkHistoryPoint {
        google_dns: google,
        cloudflare_dns: cloudflare,
        riot_games: riot,
    });
    if history.len() > MAX_TELEMETRY_HISTORY {
        history.remove(0);
    }
    true
}

#[derive(Clone, Copy)]
pub struct TelemetrySignals {
    pub rust_status: ReadSignal<Option<SystemMetrics>>,
    pub set_rust_status: WriteSignal<Option<SystemMetrics>>,
    pub java_status: ReadSignal<Option<InfrastructureMetrics>>,
    pub set_java_status: WriteSignal<Option<InfrastructureMetrics>>,
    pub telemetry_history: ReadSignal<Vec<TelemetryPoint>>,
    pub set_telemetry_history: WriteSignal<Vec<TelemetryPoint>>,
    pub network_status: ReadSignal<Option<NetworkMetrics>>,
    pub set_network_status: WriteSignal<Option<NetworkMetrics>>,
    pub network_history: ReadSignal<Vec<NetworkHistoryPoint>>,
    pub set_network_history: WriteSignal<Vec<NetworkHistoryPoint>>,
}

pub fn use_telemetry(is_visible: Signal<bool>) -> TelemetrySignals {
    let (rust_status, set_rust_status) = signal(None::<SystemMetrics>);
    let (java_status, set_java_status) = signal(None::<InfrastructureMetrics>);
    let (telemetry_history, set_telemetry_history) = signal(Vec::<TelemetryPoint>::new());
    let (network_status, set_network_status) = signal(None::<NetworkMetrics>);
    let (network_history, set_network_history) = signal(Vec::<NetworkHistoryPoint>::new());

    #[cfg(target_arch = "wasm32")]
    {
        use crate::api::api_url;
        use gloo_net::http::Request;

        // One-time history fetches on visibility change to true
        Effect::new(move |_| {
            if is_visible.get() {
                // Fetch Rust Telemetry History
                leptos::task::spawn_local(async move {
                    let url = api_url("rust", "/api/status/history");
                    if let Ok(resp) = Request::get(&url).send().await {
                        if let Ok(data) = resp.json::<Vec<TelemetryPoint>>().await {
                            set_telemetry_history.set(data);
                        }
                    }
                });

                // Fetch Rust Network History
                leptos::task::spawn_local(async move {
                    let url = api_url("rust", "/api/status/network/history");
                    if let Ok(resp) = Request::get(&url).send().await {
                        if let Ok(data) = resp.json::<Vec<NetworkHistoryPoint>>().await {
                            set_network_history.set(data);
                        }
                    }
                });
            }
        });

        // Polling loop with interval handles cleaned up on tab hide or unmount
        Effect::new(move |_| {
            if !is_visible.get() {
                return;
            }

            let Some(window) = web_sys::window() else {
                return;
            };

            // 1. Rust Hardware Status (every 5s)
            let fetch_rust = move || {
                leptos::task::spawn_local(async move {
                    let url = api_url("rust", "/api/status");
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            if let Ok(data) = resp.json::<SystemMetrics>().await {
                                let cpu = data.cpu_usage_percent;
                                let mem = calculate_memory_percent(
                                    data.memory_used_mb,
                                    data.memory_total_mb,
                                );
                                set_rust_status.set(Some(data));
                                set_telemetry_history.update(|h| {
                                    append_telemetry_point(h, cpu, mem);
                                });
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_2(
                                &"Error fetching from Rust API:".into(),
                                &e.to_string().into(),
                            );
                        }
                    }
                });
            };
            fetch_rust();

            let rust_closure = Closure::<dyn FnMut()>::new(fetch_rust);
            let rust_interval = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    rust_closure.as_ref().unchecked_ref(),
                    5000,
                )
                .ok();
            rust_closure.forget();

            // 2. Rust Network Status (every 5s)
            let fetch_network = move || {
                leptos::task::spawn_local(async move {
                    let url = api_url("rust", "/api/status/network");
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            if let Ok(data) = resp.json::<NetworkMetrics>().await {
                                let google = data.google_dns.latency_ms;
                                let cloudflare = data.cloudflare_dns.latency_ms;
                                let riot = data.riot_games.latency_ms;
                                set_network_status.set(Some(data));
                                set_network_history.update(|h| {
                                    append_network_history_point(h, google, cloudflare, riot);
                                });
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_2(
                                &"Error fetching from Rust Network API:".into(),
                                &e.to_string().into(),
                            );
                        }
                    }
                });
            };
            fetch_network();

            let net_closure = Closure::<dyn FnMut()>::new(fetch_network);
            let net_interval = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    net_closure.as_ref().unchecked_ref(),
                    5000,
                )
                .ok();
            net_closure.forget();

            // 3. Java JVM Status (every 10s)
            let fetch_java = move || {
                leptos::task::spawn_local(async move {
                    let url = api_url("java", "/api/infrastructure/metrics");
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            if let Ok(data) = resp.json::<InfrastructureMetrics>().await {
                                set_java_status.set(Some(data));
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_2(
                                &"Error fetching from Java API:".into(),
                                &e.to_string().into(),
                            );
                        }
                    }
                });
            };
            fetch_java();

            let java_closure = Closure::<dyn FnMut()>::new(fetch_java);
            let java_interval = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    java_closure.as_ref().unchecked_ref(),
                    10000,
                )
                .ok();
            java_closure.forget();

            // Cleanup when visibility changes to false or on effect rerun
            on_cleanup(move || {
                if let Some(w) = web_sys::window() {
                    if let Some(id) = rust_interval {
                        w.clear_interval_with_handle(id);
                    }
                    if let Some(id) = net_interval {
                        w.clear_interval_with_handle(id);
                    }
                    if let Some(id) = java_interval {
                        w.clear_interval_with_handle(id);
                    }
                }
            });
        });
    }

    TelemetrySignals {
        rust_status,
        set_rust_status,
        java_status,
        set_java_status,
        telemetry_history,
        set_telemetry_history,
        network_status,
        set_network_status,
        network_history,
        set_network_history,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_percent() {
        assert_eq!(calculate_memory_percent(512, 1024), 50.0);
        assert_eq!(calculate_memory_percent(256, 1024), 25.0);
        assert_eq!(calculate_memory_percent(512, 0), 0.0);
    }

    #[test]
    fn test_append_telemetry_point_dedup_and_cap() {
        let mut history = Vec::new();

        // Append initial
        assert!(append_telemetry_point(&mut history, 10.0, 20.0));
        assert_eq!(history.len(), 1);

        // Append identical point -> should be skipped
        assert!(!append_telemetry_point(&mut history, 10.0, 20.0));
        assert_eq!(history.len(), 1);

        // Append different point -> succeeds
        assert!(append_telemetry_point(&mut history, 15.0, 25.0));
        assert_eq!(history.len(), 2);

        // Fill up to cap of 20
        for i in 2..20 {
            append_telemetry_point(&mut history, i as f32, i as f32);
        }
        assert_eq!(history.len(), 20);
        assert_eq!(history[0].cpu, 10.0);

        // Append 21st point -> oldest dropped, length stays 20
        append_telemetry_point(&mut history, 99.0, 99.0);
        assert_eq!(history.len(), 20);
        assert_eq!(history[0].cpu, 15.0);
        assert_eq!(history.last().unwrap().cpu, 99.0);
    }

    #[test]
    fn test_append_network_history_point() {
        let mut history = Vec::new();
        assert!(append_network_history_point(&mut history, 10.0, 12.0, 25.0));
        assert_eq!(history.len(), 1);

        // Skip identical
        assert!(!append_network_history_point(
            &mut history,
            10.0,
            12.0,
            25.0
        ));
        assert_eq!(history.len(), 1);

        // Add different
        assert!(append_network_history_point(&mut history, 11.0, 12.0, 25.0));
        assert_eq!(history.len(), 2);

        for i in 2..25 {
            append_network_history_point(&mut history, i as f32, 10.0, 20.0);
        }
        assert_eq!(history.len(), 20);
    }
}
