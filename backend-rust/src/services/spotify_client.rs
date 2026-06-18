use axum::Json;
use reqwest::Client;
use std::env;
use base64::Engine;
use crate::models::{SpotifyStatus, SpotifyTokenResponse, AppState};
use crate::utils::is_local_runtime;

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

// Save the most recently observed good track so we can keep showing it
// if Spotify later becomes unreachable.
async fn remember(state: &AppState, status: &SpotifyStatus) {
    *state.spotify_cache.write().await = Some(status.clone());
}

// On a runtime failure, serve the last-known track as "recently played"
// instead of going grey/OFFLINE. Falls back to OFFLINE only on cold start.
async fn cached_or_offline(state: &AppState, reason: &str) -> Json<SpotifyStatus> {
    if let Some(cached) = state.spotify_cache.read().await.clone() {
        return Json(SpotifyStatus {
            is_playing: false,
            is_recently_played: true,
            progress_ms: 0,
            ..cached
        });
    }
    Json(spotify_offline_fallback(reason))
}

fn get_api_base_url() -> String {
    env::var("SPOTIFY_API_BASE_URL").unwrap_or_else(|_| "https://api.spotify.com".to_string())
}

fn get_auth_base_url() -> String {
    env::var("SPOTIFY_AUTH_BASE_URL").unwrap_or_else(|_| "https://accounts.spotify.com".to_string())
}

async fn get_recently_played(client: &Client, access_token: &str, state: &AppState) -> Json<SpotifyStatus> {
    let url = format!("{}/v1/me/player/recently-played?limit=1", get_api_base_url());
    let recently_played_res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await;

    if let Ok(res) = recently_played_res {
        if res.status() == 200 {
            if let Ok(recent_data) = res.json::<serde_json::Value>().await {
                if let Some(item) = recent_data["items"].get(0) {
                    let track = &item["track"];
                    let title = track["name"].as_str().unwrap_or("Unknown").to_string();
                    let artist = track["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string();
                    let album_art = track["album"]["images"][0]["url"].as_str().unwrap_or("").to_string();
                    let track_url = track["external_urls"]["spotify"].as_str().unwrap_or("").to_string();
                    let duration_ms = track["duration_ms"].as_u64().unwrap_or(0);

                    let status = SpotifyStatus {
                        is_playing: false,
                        is_recently_played: true,
                        title,
                        artist,
                        album_art,
                        progress_ms: 0,
                        duration_ms,
                        track_url,
                    };
                    remember(state, &status).await;
                    return Json(status);
                }
            }
        }
    }

    cached_or_offline(state, "No active session").await
}

pub async fn get_spotify_status(state: &AppState) -> Json<SpotifyStatus> {
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
            return Json(SpotifyStatus {
                is_playing: false,
                is_recently_played: false,
                title: "Local Mode".to_string(),
                artist: "Spotify credentials not configured".to_string(),
                album_art: "".to_string(),
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
            });
        }
        _ => {
            tracing::warn!("Missing Spotify credentials");
            return Json(SpotifyStatus {
                is_playing: false,
                is_recently_played: false,
                title: "Offline".to_string(),
                artist: "Spotify credentials unavailable".to_string(),
                album_art: "".to_string(),
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
            });
        }
    };

    let client = Client::new();

    // 1. Trade the refresh token for a live access token
    let auth_header = format!(
        "Basic {}",
        base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret))
    );

    let token_url = format!("{}/api/token", get_auth_base_url());
    let token_request = match client
        .post(&token_url)
        .header("Authorization", auth_header)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
        ])
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify token request error");
            return cached_or_offline(state, "Network Error").await;
        }
    };

    // Extract the raw text from Spotify before trying to decode it
    let raw_response = match token_request.text().await {
        Ok(txt) => txt,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify token text read error");
            return cached_or_offline(state, "Text Read Error").await;
        }
    };

    // Try to parse the JSON gracefully
    let token_res: SpotifyTokenResponse = match serde_json::from_str(&raw_response) {
        Ok(parsed) => parsed,
        Err(_) => {
            // If it fails, print Spotify's exact error
            tracing::error!(response = %raw_response, "Spotify auth error");
            
            // Return a safe "Offline" state to React so the UI doesn't crash
            return cached_or_offline(state, "Auth Error").await;
        }
    };

    // 2. Query the "Currently Playing" endpoint
    let playing_url = format!("{}/v1/me/player/currently-playing", get_api_base_url());
    let playing_res = match client
        .get(&playing_url)
        .header("Authorization", format!("Bearer {}", token_res.access_token))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify currently playing request error");
            return cached_or_offline(state, "API Error").await;
        }
    };

    // 3. Parse the data. If nothing is playing, check recently played.
    if playing_res.status() == 204 || playing_res.status() == 202 {
        return get_recently_played(&client, &token_res.access_token, state).await;
    }

    let track_data: serde_json::Value = match playing_res.json().await {
        Ok(json) => json,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify currently playing JSON parse error");
            return get_recently_played(&client, &token_res.access_token, state).await;
        }
    };
    
    // Safely extract JSON fields
    let is_playing = track_data["is_playing"].as_bool().unwrap_or(false);
    let title = track_data["item"]["name"].as_str().unwrap_or("Unknown").to_string();
    let artist = track_data["item"]["artists"][0]["name"].as_str().unwrap_or("Unknown").to_string();
    let album_art = track_data["item"]["album"]["images"][0]["url"].as_str().unwrap_or("").to_string();
    let track_url = track_data["item"]["external_urls"]["spotify"].as_str().unwrap_or("").to_string();
    
    let progress_ms = track_data["progress_ms"].as_u64().unwrap_or(0);
    let duration_ms = track_data["item"]["duration_ms"].as_u64().unwrap_or(0);

    let status = SpotifyStatus {
        is_playing,
        is_recently_played: !is_playing,
        title,
        artist,
        album_art,
        progress_ms,
        duration_ms,
        track_url,
    };
    remember(state, &status).await;
    Json(status)
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[tokio::test]
    async fn test_get_spotify_status_missing_credentials_prod() {
        let _guard = ENV_MUTEX.lock().unwrap();
        clear_env();
        unsafe {
            env::set_var("APP_ENV", "production");
        }

        let res = get_spotify_status().await;
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

        let res = get_spotify_status().await;
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

        let res = get_spotify_status().await;
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

        let res = get_spotify_status().await;
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

        let res = get_spotify_status().await;
        assert_eq!(res.title, "Auth Error");
        assert_eq!(res.artist, "Service temporarily unavailable");

        unsafe {
            env::remove_var("SPOTIFY_AUTH_BASE_URL");
        }
    }
}

