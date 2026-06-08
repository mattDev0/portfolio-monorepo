use reqwest::Client;
use std::env;
use tokio::time::{sleep, Duration};
use crate::models::{AppState, NetworkMetrics, NetworkTarget, NetworkHistoryPoint};
use crate::utils::url_encode;

pub fn start_network_monitor(state: AppState) {
    let net_metrics_clone = state.network_metrics.clone();
    let net_history_clone = state.network_history.clone();

    tokio::spawn(async move {
        let client = Client::new();
        let prometheus_url = env::var("PROMETHEUS_URL")
            .unwrap_or_else(|_| "http://localhost:9090".to_string());

        loop {
            let net_metrics = match fetch_network_metrics(&client, &prometheus_url).await {
                Some(m) => m,
                None => generate_mock_network_metrics(),
            };

            {
                let mut guard = net_metrics_clone.write().await;
                *guard = net_metrics.clone();
            }

            {
                let mut history_guard = net_history_clone.write().await;
                history_guard.push(NetworkHistoryPoint {
                    google_dns: net_metrics.google_dns.latency_ms,
                    cloudflare_dns: net_metrics.cloudflare_dns.latency_ms,
                    riot_games: net_metrics.riot_games.latency_ms,
                });
                if history_guard.len() > 20 {
                    history_guard.remove(0);
                }
            }

            sleep(Duration::from_secs(5)).await;
        }
    });
}

async fn query_prometheus_metric(client: &Client, prometheus_url: &str, query: &str) -> Option<serde_json::Value> {
    let url = format!("{}/api/v1/query?query={}", prometheus_url, url_encode(query));
    let res = client
        .get(&url)
        .send()
        .await
        .ok()?;
    
    if res.status() == 200 {
        res.json::<serde_json::Value>().await.ok()
    } else {
        None
    }
}

fn get_metric_value_for_instance(json: &serde_json::Value, instance: &str) -> Option<f32> {
    let results = json.get("data")?.get("result")?.as_array()?;
    for result in results {
        let metric_instance = result.get("metric")?.get("instance")?.as_str()?;
        if metric_instance == instance {
            let value_arr = result.get("value")?.as_array()?;
            let value_str = value_arr.get(1)?.as_str()?;
            return value_str.parse::<f32>().ok();
        }
    }
    None
}

async fn fetch_network_metrics(client: &Client, prometheus_url: &str) -> Option<NetworkMetrics> {
    let duration_json = query_prometheus_metric(client, prometheus_url, "probe_duration_seconds{job=\"network_latency_ping\"}").await?;
    let success_json = query_prometheus_metric(client, prometheus_url, "probe_success{job=\"network_latency_ping\"}").await?;

    let google_lat = get_metric_value_for_instance(&duration_json, "8.8.8.8")? * 1000.0;
    let google_suc = get_metric_value_for_instance(&success_json, "8.8.8.8")?.round() as i32;

    let cloudflare_lat = get_metric_value_for_instance(&duration_json, "1.1.1.1")? * 1000.0;
    let cloudflare_suc = get_metric_value_for_instance(&success_json, "1.1.1.1")?.round() as i32;

    let riot_lat = get_metric_value_for_instance(&duration_json, "104.160.131.3")? * 1000.0;
    let riot_suc = get_metric_value_for_instance(&success_json, "104.160.131.3")?.round() as i32;

    Some(NetworkMetrics {
        google_dns: NetworkTarget {
            target: "8.8.8.8".to_string(),
            name: "Google DNS".to_string(),
            latency_ms: (google_lat * 10.0).round() / 10.0,
            status: if google_suc == 1 { "online".to_string() } else { "offline".to_string() },
        },
        cloudflare_dns: NetworkTarget {
            target: "1.1.1.1".to_string(),
            name: "Cloudflare DNS".to_string(),
            latency_ms: (cloudflare_lat * 10.0).round() / 10.0,
            status: if cloudflare_suc == 1 { "online".to_string() } else { "offline".to_string() },
        },
        riot_games: NetworkTarget {
            target: "104.160.131.3".to_string(),
            name: "Riot Games NA".to_string(),
            latency_ms: (riot_lat * 10.0).round() / 10.0,
            status: if riot_suc == 1 { "online".to_string() } else { "offline".to_string() },
        },
    })
}

fn generate_mock_network_metrics() -> NetworkMetrics {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let osc1 = (((now % 10000) as f32 / 10000.0) * 2.0 * std::f32::consts::PI).sin();
    let osc2 = (((now % 7000) as f32 / 7000.0) * 2.0 * std::f32::consts::PI).cos();
    
    let google_latency = 12.0 + osc1 * 1.8;
    let cloudflare_latency = 8.5 + osc2 * 1.2;
    let riot_latency = 39.0 + osc1 * 3.5;

    NetworkMetrics {
        google_dns: NetworkTarget {
            target: "8.8.8.8".to_string(),
            name: "Google DNS".to_string(),
            latency_ms: (google_latency * 10.0).round() / 10.0,
            status: "online".to_string(),
        },
        cloudflare_dns: NetworkTarget {
            target: "1.1.1.1".to_string(),
            name: "Cloudflare DNS".to_string(),
            latency_ms: (cloudflare_latency * 10.0).round() / 10.0,
            status: "online".to_string(),
        },
        riot_games: NetworkTarget {
            target: "104.160.131.3".to_string(),
            name: "Riot Games NA".to_string(),
            latency_ms: (riot_latency * 10.0).round() / 10.0,
            status: "online".to_string(),
        },
    }
}
