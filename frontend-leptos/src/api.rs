use std::collections::HashSet;

pub const RUST_PROD_URL: &str = "https://portfolio-rust-api-850595626708.us-central1.run.app";
pub const JAVA_PROD_URL: &str = "https://portfolio-java-api-850595626708.us-central1.run.app";

pub const DEFAULT_LOCAL_RUST_URL: &str = "http://localhost:8080";
pub const DEFAULT_LOCAL_JAVA_URL: &str = "http://localhost:8081";

pub fn is_local_runtime_pure(
    local_dev_flag: Option<&str>,
    hostname: &str,
    debug_mode: bool,
) -> bool {
    let truthy: HashSet<&str> = ["1", "true", "yes", "local", "dev", "development"]
        .into_iter()
        .collect();
    let falsy: HashSet<&str> = ["0", "false", "no", "prod", "production"]
        .into_iter()
        .collect();
    let local_hosts: HashSet<&str> = ["localhost", "127.0.0.1", "::1", "0.0.0.0"]
        .into_iter()
        .collect();

    let normalized_flag = local_dev_flag.unwrap_or("").trim().to_lowercase();

    if truthy.contains(normalized_flag.as_str()) {
        return true;
    }

    if !falsy.contains(normalized_flag.as_str())
        && (debug_mode || local_hosts.contains(hostname.trim()))
    {
        return true;
    }

    false
}

pub fn trim_trailing_slash(s: &str) -> &str {
    s.trim_end_matches('/')
}

pub fn normalize_path(path: &str) -> String {
    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{}", path)
    }
}

pub fn resolve_api_base_url(
    service: &str,
    is_local: bool,
    rust_local_override: Option<&str>,
    java_local_override: Option<&str>,
) -> String {
    if is_local {
        match service {
            "rust" => trim_trailing_slash(rust_local_override.unwrap_or(DEFAULT_LOCAL_RUST_URL))
                .to_string(),
            "java" => trim_trailing_slash(java_local_override.unwrap_or(DEFAULT_LOCAL_JAVA_URL))
                .to_string(),
            _ => String::new(),
        }
    } else {
        match service {
            "rust" => RUST_PROD_URL.to_string(),
            "java" => JAVA_PROD_URL.to_string(),
            _ => String::new(),
        }
    }
}

pub fn construct_api_url_pure(
    service: &str,
    path: &str,
    is_local: bool,
    rust_local_override: Option<&str>,
    java_local_override: Option<&str>,
) -> String {
    let base = resolve_api_base_url(service, is_local, rust_local_override, java_local_override);
    let norm_path = normalize_path(path);
    format!("{}{}", base, norm_path)
}

pub fn get_browser_hostname() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.location().hostname().ok())
            .unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        String::new()
    }
}

pub fn is_local_runtime() -> bool {
    let flag = option_env!("PORTFOLIO_LOCAL_DEV");
    let hostname = get_browser_hostname();
    let is_debug = cfg!(debug_assertions);
    is_local_runtime_pure(flag, &hostname, is_debug)
}

pub fn api_url(service: &str, path: &str) -> String {
    let is_local = is_local_runtime();
    let rust_override = option_env!("PORTFOLIO_LOCAL_RUST_API_BASE_URL");
    let java_override = option_env!("PORTFOLIO_LOCAL_JAVA_API_BASE_URL");
    construct_api_url_pure(service, path, is_local, rust_override, java_override)
}

/// GET a JSON endpoint with a timeout. Network errors, timeouts, non-2xx
/// responses and bad JSON all come back as `Err`, so callers can count them
/// as failed polls instead of silently keeping old data.
#[cfg(target_arch = "wasm32")]
pub async fn fetch_json<T: serde::de::DeserializeOwned>(
    url: &str,
    timeout_ms: u32,
) -> Result<T, String> {
    use gloo_net::http::Request;

    let signal = web_sys::AbortSignal::timeout_with_u32(timeout_ms);
    let resp = Request::get(url)
        .abort_signal(Some(&signal))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    resp.json::<T>().await.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_local_runtime_truthy_flags() {
        for flag in &[
            "1",
            "true",
            "yes",
            "local",
            "dev",
            "development",
            " TRUE ",
            "Local",
        ] {
            assert!(
                is_local_runtime_pure(Some(flag), "mattdev0.tech", false),
                "Expected {} to be local",
                flag
            );
        }
    }

    #[test]
    fn test_is_local_runtime_falsy_flags() {
        for flag in &["0", "false", "no", "prod", "production", " FALSE "] {
            // Even on localhost or debug, explicit falsy flag forces production
            assert!(
                !is_local_runtime_pure(Some(flag), "localhost", true),
                "Expected {} to be prod",
                flag
            );
        }
    }

    #[test]
    fn test_is_local_runtime_hostnames() {
        for host in &["localhost", "127.0.0.1", "::1", "0.0.0.0"] {
            assert!(
                is_local_runtime_pure(None, host, false),
                "Expected host {} to be local",
                host
            );
        }
        assert!(
            !is_local_runtime_pure(None, "mattdev0.tech", false),
            "Expected mattdev0.tech to be prod"
        );
    }

    #[test]
    fn test_is_local_runtime_debug_mode() {
        assert!(is_local_runtime_pure(None, "mattdev0.tech", true));
        assert!(!is_local_runtime_pure(None, "mattdev0.tech", false));
    }

    #[test]
    fn test_trailing_slash_trimming() {
        assert_eq!(
            trim_trailing_slash("http://localhost:8080/"),
            "http://localhost:8080"
        );
        assert_eq!(
            trim_trailing_slash("http://localhost:8080"),
            "http://localhost:8080"
        );
        assert_eq!(
            trim_trailing_slash("https://example.com///"),
            "https://example.com"
        );
    }

    #[test]
    fn test_path_leading_slash() {
        assert_eq!(normalize_path("status"), "/status");
        assert_eq!(normalize_path("/status"), "/status");
    }

    #[test]
    fn test_construct_api_urls_local() {
        assert_eq!(
            construct_api_url_pure("rust", "/status", true, None, None),
            "http://localhost:8080/status"
        );
        assert_eq!(
            construct_api_url_pure("rust", "status", true, None, None),
            "http://localhost:8080/status"
        );
        assert_eq!(
            construct_api_url_pure("java", "/api/github/activity", true, None, None),
            "http://localhost:8081/api/github/activity"
        );
        assert_eq!(
            construct_api_url_pure(
                "rust",
                "/api/status",
                true,
                Some("http://192.168.1.50:9000/"),
                None
            ),
            "http://192.168.1.50:9000/api/status"
        );
    }

    #[test]
    fn test_construct_api_urls_production() {
        assert_eq!(
            construct_api_url_pure("rust", "/api/status", false, None, None),
            format!("{}/api/status", RUST_PROD_URL)
        );
        assert_eq!(
            construct_api_url_pure("java", "api/infrastructure/metrics", false, None, None),
            format!("{}/api/infrastructure/metrics", JAVA_PROD_URL)
        );
    }
}
