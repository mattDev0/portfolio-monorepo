use axum::Json;
use reqwest::{Client, StatusCode};
use std::env;
use std::time::{Duration, Instant};
use base64::Engine;
use crate::models::{SpotifyStatus, SpotifyTokenResponse, CachedToken, AppState};
use crate::utils::is_local_runtime;

// Collapse concurrent visitor polls into a single upstream call, and stop a
// persistent upstream failure from being retried on every inbound request.
const FRESH_TTL: Duration = Duration::from_secs(5);
// Refresh slightly before Spotify expires the access token.
const TOKEN_SAFETY_MARGIN: Duration = Duration::from_secs(60);
const RECENTLY_PLAYED_SCOPE: &str = "user-read-recently-played";

fn spotify_offline_fallback(reason: &str) -> SpotifyStatus {
    SpotifyStatus {
        is_playing: false,
        is_recently_played: false,
        title: "Offline".to_string(),
        artist: reason.to_string(),
        album_art: "".to_string(),
        progress_ms: 0,
        duration_ms: 0,
        track_url: "".to_string(),
    }
}

// Keep log lines bounded and UTF-8 safe.
fn truncate(s: &str) -> String {
    let t = s.trim();
    match t.char_indices().nth(200) {
        Some((i, _)) => format!("{}...", &t[..i]),
        None => t.to_string(),
    }
}

// Save the most recently observed good track so we can keep showing it
// if Spotify later becomes unreachable.
async fn remember(state: &AppState, status: &SpotifyStatus) {
    *state.spotify_cache.write().await = Some(status.clone());
}

// On a runtime failure, serve the last-known track as "recently played"
// instead of going grey/OFFLINE. Falls back to OFFLINE only on cold start.
async fn cached_or_offline(state: &AppState, reason: &str) -> SpotifyStatus {
    if let Some(cached) = state.spotify_cache.read().await.clone() {
        return SpotifyStatus {
            is_playing: false,
            is_recently_played: true,
            progress_ms: 0,
            ..cached
        };
    }
    spotify_offline_fallback(reason)
}

async fn read_fresh(state: &AppState) -> Option<SpotifyStatus> {
    let guard = state.spotify_fresh.read().await;
    match guard.as_ref() {
        Some((status, stored_at)) if stored_at.elapsed() < FRESH_TTL => Some(status.clone()),
        _ => None,
    }
}

async fn store_fresh(state: &AppState, status: &SpotifyStatus) {
    *state.spotify_fresh.write().await = Some((status.clone(), Instant::now()));
}

// One shared client (and therefore one connection pool) for the whole process.
// Previously a fresh Client - and a fresh TLS handshake - was built per request.
fn http_client() -> &'static Client {
    static CLIENT: std::sync::OnceLock<Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new())
    })
}

fn get_api_base_url() -> String {
    env::var("SPOTIFY_API_BASE_URL").unwrap_or_else(|_| "https://api.spotify.com".to_string())
}

fn get_auth_base_url() -> String {
    env::var("SPOTIFY_AUTH_BASE_URL").unwrap_or_else(|_| "https://accounts.spotify.com".to_string())
}

// Trade the refresh token for an access token, reusing the cached one until it
// is close to expiry. Previously every inbound request performed this exchange.
async fn get_access_token(
    state: &AppState,
    client: &Client,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<CachedToken, String> {
    let cached = state.spotify_token.read().await.clone();
    if let Some(token) = cached
        .filter(|t| t.expires_at.saturating_duration_since(Instant::now()) > TOKEN_SAFETY_MARGIN)
    {
        return Ok(token);
    }

    let auth_header = format!(
        "Basic {}",
        base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret))
    );

    let token_url = format!("{}/api/token", get_auth_base_url());
    let response = client
        .post(&token_url)
        .header("Authorization", auth_header)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "Spotify token request error");
            "Network Error".to_string()
        })?;

    let status = response.status();
    let raw_response = response.text().await.map_err(|e| {
        tracing::error!(error = ?e, status = %status, "Spotify token text read error");
        "Text Read Error".to_string()
    })?;

    let parsed: SpotifyTokenResponse = serde_json::from_str(&raw_response).map_err(|_| {
        tracing::error!(status = %status, response = %truncate(&raw_response), "Spotify auth error");
        "Auth Error".to_string()
    })?;

    let lifetime = if parsed.expires_in == 0 { 3600 } else { parsed.expires_in };
    let token = CachedToken {
        access_token: parsed.access_token,
        scope: parsed.scope,
        expires_at: Instant::now() + Duration::from_secs(lifetime),
    };
    *state.spotify_token.write().await = Some(token.clone());
    Ok(token)
}

async fn get_recently_played(client: &Client, token: &CachedToken, state: &AppState) -> SpotifyStatus {
    let url = format!("{}/v1/me/player/recently-played?limit=1", get_api_base_url());
    let response = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify recently played request error");
            return cached_or_offline(state, "No active session").await;
        }
    };

    let status = response.status();
    if status != StatusCode::OK {
        let body = response.text().await.unwrap_or_default();
        tracing::warn!(status = %status, body = %truncate(&body), "Spotify recently played returned a non-200");
        return cached_or_offline(state, "No active session").await;
    }

    let recent_data: serde_json::Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify recently played JSON parse error");
            return cached_or_offline(state, "No active session").await;
        }
    };

    let Some(item) = recent_data["items"].get(0) else {
        return cached_or_offline(state, "No active session").await;
    };

    let track = &item["track"];
    let status = SpotifyStatus {
        is_playing: false,
        is_recently_played: true,
        title: track["name"].as_str().unwrap_or("Unknown").to_string(),
        artist: track["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string(),
        album_art: track["album"]["images"][0]["url"].as_str().unwrap_or("").to_string(),
        progress_ms: 0,
        duration_ms: track["duration_ms"].as_u64().unwrap_or(0),
        track_url: track["external_urls"]["spotify"].as_str().unwrap_or("").to_string(),
    };
    remember(state, &status).await;
    status
}

// Nothing is currently playing. Only ask for recently-played if the token was
// actually granted that scope - otherwise it is a guaranteed 403 and simply
// doubles our upstream request rate.
async fn no_active_session(client: &Client, token: &CachedToken, state: &AppState) -> SpotifyStatus {
    // Only skip when Spotify actually told us the granted scopes and this one
    // is absent. If the scope list is unknown, still try rather than assume.
    let scope_known = !token.scope.trim().is_empty();
    let has_scope = token.scope.split_whitespace().any(|s| s == RECENTLY_PLAYED_SCOPE);
    if !scope_known || has_scope {
        return get_recently_played(client, token, state).await;
    }
    tracing::debug!(
        scope = %token.scope,
        "Skipping recently-played: token lacks the {} scope",
        RECENTLY_PLAYED_SCOPE
    );
    cached_or_offline(state, "No active session").await
}

async fn fetch_spotify_status(state: &AppState) -> SpotifyStatus {
    let spotify_credentials = (
        env::var("SPOTIFY_CLIENT_ID"),
        env::var("SPOTIFY_CLIENT_SECRET"),
        env::var("SPOTIFY_REFRESH_TOKEN"),
    );

    let (client_id, client_secret, refresh_token) = match spotify_credentials {
        (Ok(client_id), Ok(client_secret), Ok(refresh_token)) => {
            (client_id, client_secret, refresh_token)
        }
        _ if is_local_runtime() => {
            return SpotifyStatus {
                is_playing: false,
                is_recently_played: false,
                title: "Local Mode".to_string(),
                artist: "Spotify credentials not configured".to_string(),
                album_art: "".to_string(),
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
            };
        }
        _ => {
            tracing::warn!("Missing Spotify credentials");
            return spotify_offline_fallback("Spotify credentials unavailable");
        }
    };

    let client = http_client();

    let token = match get_access_token(state, client, &client_id, &client_secret, &refresh_token).await {
        Ok(token) => token,
        Err(reason) => return cached_or_offline(state, &reason).await,
    };

    let playing_url = format!("{}/v1/me/player/currently-playing", get_api_base_url());
    let response = match client
        .get(&playing_url)
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify currently playing request error");
            return cached_or_offline(state, "API Error").await;
        }
    };

    let status = response.status();

    if status == StatusCode::NO_CONTENT || status == StatusCode::ACCEPTED {
        return no_active_session(client, &token, state).await;
    }

    if status == StatusCode::UNAUTHORIZED {
        tracing::warn!(status = %status, "Spotify rejected the access token; dropping the cached token");
        *state.spotify_token.write().await = None;
        return cached_or_offline(state, "Auth Error").await;
    }

    if status == StatusCode::TOO_MANY_REQUESTS {
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unset")
            .to_string();
        tracing::warn!(status = %status, retry_after = %retry_after, "Spotify rate limited currently-playing");
        return cached_or_offline(state, "Rate Limited").await;
    }

    if status == StatusCode::FORBIDDEN {
        tracing::warn!(status = %status, scope = %token.scope, "Spotify refused currently-playing");
        return cached_or_offline(state, "Scope Error").await;
    }

    let raw_response = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            tracing::error!(error = ?e, status = %status, "Spotify currently playing body read error");
            return cached_or_offline(state, "API Error").await;
        }
    };

    // Spotify occasionally answers with a 2xx and no body. Treat that as "no
    // session" rather than letting serde fail on an empty string.
    if raw_response.trim().is_empty() {
        tracing::warn!(status = %status, "Spotify returned an empty currently-playing body");
        return no_active_session(client, &token, state).await;
    }

    let track_data: serde_json::Value = match serde_json::from_str(&raw_response) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!(
                error = %e,
                status = %status,
                body = %truncate(&raw_response),
                "Spotify currently playing JSON parse error"
            );
            return no_active_session(client, &token, state).await;
        }
    };

    // An advert or podcast episode yields a 200 with a null item.
    if track_data["item"].is_null() {
        return no_active_session(client, &token, state).await;
    }

    let is_playing = track_data["is_playing"].as_bool().unwrap_or(false);
    let status = SpotifyStatus {
        is_playing,
        is_recently_played: !is_playing,
        title: track_data["item"]["name"].as_str().unwrap_or("Unknown").to_string(),
        artist: track_data["item"]["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string(),
        album_art: track_data["item"]["album"]["images"][0]["url"].as_str().unwrap_or("").to_string(),
        progress_ms: track_data["progress_ms"].as_u64().unwrap_or(0),
        duration_ms: track_data["item"]["duration_ms"].as_u64().unwrap_or(0),
        track_url: track_data["item"]["external_urls"]["spotify"].as_str().unwrap_or("").to_string(),
    };
    remember(state, &status).await;
    status
}

pub async fn get_spotify_status(state: &AppState) -> Json<SpotifyStatus> {
    if let Some(fresh) = read_fresh(state).await {
        return Json(fresh);
    }
    let status = fetch_spotify_status(state).await;
    store_fresh(state, &status).await;
    Json(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use serde_json::json;
    use std::sync::Mutex;

    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn clear_env() {
        unsafe {
            env::remove_var("SPOTIFY_CLIENT_ID");
            env::remove_var("SPOTIFY_CLIENT_SECRET");
            env::remove_var("SPOTIFY_REFRESH_TOKEN");
            env::remove_var("APP_ENV");
        }
    }

    fn test_state() -> AppState {
        AppState {
            metrics: Arc::new(tokio::sync::RwLock::new(Default::default())),
            history: Arc::new(tokio::sync::RwLock::new(Default::default())),
            network_metrics: Arc::new(tokio::sync::RwLock::new(Default::default())),
            network_history: Arc::new(tokio::sync::RwLock::new(Default::default())),
            spotify_cache: Arc::new(tokio::sync::RwLock::new(None)),
            spotify_token: Arc::new(tokio::sync::RwLock::new(None)),
            spotify_fresh: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    #[tokio::test]
    async fn test_get_spotify_status_missing_credentials_prod() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        unsafe {
            env::set_var("APP_ENV", "production");
        }

        let state = test_state();
        let res = get_spotify_status(&state).await;
        assert_eq!(res.title, "Offline");
        assert_eq!(res.artist, "Spotify credentials unavailable");
    }

    #[tokio::test]
    async fn test_get_spotify_status_missing_credentials_local() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        unsafe {
            env::set_var("APP_ENV", "local");
        }

        let state = test_state();
        let res = get_spotify_status(&state).await;
        assert_eq!(res.title, "Local Mode");
        assert_eq!(res.artist, "Spotify credentials not configured");
    }

    #[tokio::test]
    async fn test_get_spotify_status_currently_playing_mocked() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        
        let auth_server = MockServer::start().await;
        let api_server = MockServer::start().await;

        unsafe {
            env::set_var("SPOTIFY_CLIENT_ID", "mock_id");
            env::set_var("SPOTIFY_CLIENT_SECRET", "mock_secret");
            env::set_var("SPOTIFY_REFRESH_TOKEN", "mock_refresh");
            env::set_var("SPOTIFY_AUTH_BASE_URL", auth_server.uri());
            env::set_var("SPOTIFY_API_BASE_URL", api_server.uri());
            env::set_var("APP_ENV", "production");
        }

        // Mock token exchange
        let token_response = json!({
            "access_token": "mock_access_token",
            "token_type": "Bearer",
            "expires_in": 3600,
            "scope": ""
        });
        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(token_response))
            .mount(&auth_server)
            .await;

        // Mock currently playing API
        let currently_playing_response = json!({
            "is_playing": true,
            "progress_ms": 5000,
            "item": {
                "name": "Mock Track Name",
                "duration_ms": 180000,
                "external_urls": {
                    "spotify": "https://spotify.com/mock-track"
                },
                "artists": [
                    { "name": "Mock Artist Name" }
                ],
                "album": {
                    "images": [
                        { "url": "https://spotify.com/mock-art.jpg" }
                    ]
                }
            }
        });
        Mock::given(method("GET"))
            .and(path("/v1/me/player/currently-playing"))
            .respond_with(ResponseTemplate::new(200).set_body_json(currently_playing_response))
            .mount(&api_server)
            .await;

        let state = test_state();
        let res = get_spotify_status(&state).await;
        assert!(res.is_playing);
        assert_eq!(res.title, "Mock Track Name");
        assert_eq!(res.artist, "Mock Artist Name");
        assert_eq!(res.album_art, "https://spotify.com/mock-art.jpg");
        assert_eq!(res.track_url, "https://spotify.com/mock-track");
        assert_eq!(res.progress_ms, 5000);
        assert_eq!(res.duration_ms, 180000);

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
            env::remove_var("SPOTIFY_API_BASE_URL");
        }
    }

    #[tokio::test]
    async fn test_get_spotify_status_recently_played_mocked() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        
        let auth_server = MockServer::start().await;
        let api_server = MockServer::start().await;

        unsafe {
            env::set_var("SPOTIFY_CLIENT_ID", "mock_id");
            env::set_var("SPOTIFY_CLIENT_SECRET", "mock_secret");
            env::set_var("SPOTIFY_REFRESH_TOKEN", "mock_refresh");
            env::set_var("SPOTIFY_AUTH_BASE_URL", auth_server.uri());
            env::set_var("SPOTIFY_API_BASE_URL", api_server.uri());
            env::set_var("APP_ENV", "production");
        }

        // Mock token exchange
        let token_response = json!({
            "access_token": "mock_access_token",
            "token_type": "Bearer",
            "expires_in": 3600,
            "scope": ""
        });
        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(token_response))
            .mount(&auth_server)
            .await;

        // Mock currently playing API returns 204 No Content
        Mock::given(method("GET"))
            .and(path("/v1/me/player/currently-playing"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&api_server)
            .await;

        // Mock recently played API
        let recently_played_response = json!({
            "items": [
                {
                    "track": {
                        "name": "Mock Recent Track",
                        "duration_ms": 200000,
                        "external_urls": {
                            "spotify": "https://spotify.com/mock-recent"
                        },
                        "artists": [
                            { "name": "Mock Recent Artist" }
                        ],
                        "album": {
                            "images": [
                                { "url": "https://spotify.com/mock-recent-art.jpg" }
                            ]
                        }
                    }
                }
            ]
        });
        Mock::given(method("GET"))
            .and(path("/v1/me/player/recently-played"))
            .respond_with(ResponseTemplate::new(200).set_body_json(recently_played_response))
            .mount(&api_server)
            .await;

        let state = test_state();
        let res = get_spotify_status(&state).await;
        assert!(!res.is_playing);
        assert!(res.is_recently_played);
        assert_eq!(res.title, "Mock Recent Track");
        assert_eq!(res.artist, "Mock Recent Artist");
        assert_eq!(res.album_art, "https://spotify.com/mock-recent-art.jpg");
        assert_eq!(res.track_url, "https://spotify.com/mock-recent");
        assert_eq!(res.duration_ms, 200000);

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
            env::remove_var("SPOTIFY_API_BASE_URL");
        }
    }

    #[tokio::test]
    async fn test_get_spotify_status_auth_returns_malformed_json() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        
        let auth_server = MockServer::start().await;

        unsafe {
            env::set_var("SPOTIFY_CLIENT_ID", "mock_id");
            env::set_var("SPOTIFY_CLIENT_SECRET", "mock_secret");
            env::set_var("SPOTIFY_REFRESH_TOKEN", "mock_refresh");
            env::set_var("SPOTIFY_AUTH_BASE_URL", auth_server.uri());
            env::set_var("APP_ENV", "production");
        }

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&auth_server)
            .await;

        let state = test_state();
        let res = get_spotify_status(&state).await;
        assert_eq!(res.title, "Offline");
        assert_eq!(res.artist, "Auth Error");

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
        }
    }

    #[tokio::test]
    async fn test_get_spotify_status_serves_cached_track_on_error() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        
        let auth_server = MockServer::start().await;

        unsafe {
            env::set_var("SPOTIFY_CLIENT_ID", "mock_id");
            env::set_var("SPOTIFY_CLIENT_SECRET", "mock_secret");
            env::set_var("SPOTIFY_REFRESH_TOKEN", "mock_refresh");
            env::set_var("SPOTIFY_AUTH_BASE_URL", auth_server.uri());
            env::set_var("APP_ENV", "production");
        }

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&auth_server)
            .await;

        let state = test_state();
        
        {
            let mut cache = state.spotify_cache.write().await;
            *cache = Some(SpotifyStatus {
                is_playing: true,
                is_recently_played: false,
                title: "Cached Song".to_string(),
                artist: "Cached Artist".to_string(),
                album_art: "cached_art_url".to_string(),
                progress_ms: 12000,
                duration_ms: 240000,
                track_url: "cached_track_url".to_string(),
            });
        }

        let res = get_spotify_status(&state).await;
        
        assert!(!res.is_playing);
        assert!(res.is_recently_played);
        assert_eq!(res.title, "Cached Song");
        assert_eq!(res.artist, "Cached Artist");
        assert_eq!(res.album_art, "cached_art_url");
        assert_eq!(res.progress_ms, 0);
        assert_eq!(res.duration_ms, 240000);
        assert_eq!(res.track_url, "cached_track_url");

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
        }
    }

    // Regression: production returned HTTP 200 with an empty body for
    // currently-playing, serde failed with "expected value line 1 column 1",
    // and every response collapsed to Offline / "No active session".
    #[tokio::test]
    async fn test_empty_currently_playing_body_falls_back_to_recently_played() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();

        let auth_server = MockServer::start().await;
        let api_server = MockServer::start().await;

        unsafe {
            env::set_var("SPOTIFY_CLIENT_ID", "mock_id");
            env::set_var("SPOTIFY_CLIENT_SECRET", "mock_secret");
            env::set_var("SPOTIFY_REFRESH_TOKEN", "mock_refresh");
            env::set_var("SPOTIFY_AUTH_BASE_URL", auth_server.uri());
            env::set_var("SPOTIFY_API_BASE_URL", api_server.uri());
            env::set_var("APP_ENV", "production");
        }

        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "mock_access_token",
                "token_type": "Bearer",
                "expires_in": 3600,
                "scope": "user-read-currently-playing user-read-recently-played"
            })))
            .mount(&auth_server)
            .await;

        // 200 OK with a completely empty body - the production failure mode.
        Mock::given(method("GET"))
            .and(path("/v1/me/player/currently-playing"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&api_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v1/me/player/recently-played"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "items": [ { "track": {
                    "name": "Recovered Track",
                    "duration_ms": 111000,
                    "external_urls": { "spotify": "https://spotify.com/recovered" },
                    "artists": [ { "name": "Recovered Artist" } ],
                    "album": { "images": [ { "url": "https://spotify.com/recovered.jpg" } ] }
                } } ]
            })))
            .mount(&api_server)
            .await;

        let state = test_state();
        let res = get_spotify_status(&state).await;

        assert_ne!(res.title, "Offline", "empty body must not collapse to Offline");
        assert!(res.is_recently_played);
        assert_eq!(res.title, "Recovered Track");

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
            env::remove_var("SPOTIFY_API_BASE_URL");
        }
    }

    // The access token is now cached, so a burst of requests must perform a
    // single token exchange rather than one per inbound request.
    #[tokio::test]
    async fn test_access_token_is_cached_across_requests() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();

        let auth_server = MockServer::start().await;
        let api_server = MockServer::start().await;

        unsafe {
            env::set_var("SPOTIFY_CLIENT_ID", "mock_id");
            env::set_var("SPOTIFY_CLIENT_SECRET", "mock_secret");
            env::set_var("SPOTIFY_REFRESH_TOKEN", "mock_refresh");
            env::set_var("SPOTIFY_AUTH_BASE_URL", auth_server.uri());
            env::set_var("SPOTIFY_API_BASE_URL", api_server.uri());
            env::set_var("APP_ENV", "production");
        }

        // expect(1) fails the test if the token endpoint is called more than once.
        Mock::given(method("POST"))
            .and(path("/api/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "mock_access_token",
                "token_type": "Bearer",
                "expires_in": 3600,
                "scope": "user-read-currently-playing"
            })))
            .expect(1)
            .mount(&auth_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v1/me/player/currently-playing"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&api_server)
            .await;

        let state = test_state();
        for _ in 0..3 {
            // Clear the short-lived response cache so each iteration really
            // exercises the token path.
            *state.spotify_fresh.write().await = None;
            let _ = get_spotify_status(&state).await;
        }

        drop(auth_server); // verifies the expect(1) assertion

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
            env::remove_var("SPOTIFY_API_BASE_URL");
        }
    }
}
