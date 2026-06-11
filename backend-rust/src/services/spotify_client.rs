use axum::Json;
use reqwest::Client;
use std::env;
use base64::Engine;
use crate::models::{SpotifyStatus, SpotifyTokenResponse};
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

async fn get_recently_played(client: &Client, access_token: &str) -> Json<SpotifyStatus> {
    let recently_played_res = client
        .get("https://api.spotify.com/v1/me/player/recently-played?limit=1")
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

                    return Json(SpotifyStatus {
                        is_playing: false,
                        is_recently_played: true,
                        title,
                        artist,
                        album_art,
                        progress_ms: 0,
                        duration_ms,
                        track_url,
                    });
                }
            }
        }
    }

    Json(spotify_offline_fallback("No active session"))
}

pub async fn get_spotify_status() -> Json<SpotifyStatus> {
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

    let token_request = match client
        .post("https://accounts.spotify.com/api/token")
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
            return Json(spotify_offline_fallback("Network Error"));
        }
    };

    // Extract the raw text from Spotify before trying to decode it
    let raw_response = match token_request.text().await {
        Ok(txt) => txt,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify token text read error");
            return Json(spotify_offline_fallback("Text Read Error"));
        }
    };

    // Try to parse the JSON gracefully
    let token_res: SpotifyTokenResponse = match serde_json::from_str(&raw_response) {
        Ok(parsed) => parsed,
        Err(_) => {
            // If it fails, print Spotify's exact error
            tracing::error!(response = %raw_response, "Spotify auth error");
            
            // Return a safe "Offline" state to React so the UI doesn't crash
            return Json(SpotifyStatus {
                is_playing: false,
                is_recently_played: false,
                title: "Auth Error".to_string(),
                artist: "Service temporarily unavailable".to_string(),
                album_art: "".to_string(),
                progress_ms: 0,
                duration_ms: 0,
                track_url: "".to_string(),
            });
        }
    };

    // 2. Query the "Currently Playing" endpoint
    let playing_res = match client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .header("Authorization", format!("Bearer {}", token_res.access_token))
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify currently playing request error");
            return Json(spotify_offline_fallback("API Error"));
        }
    };

    // 3. Parse the data. If nothing is playing, check recently played.
    if playing_res.status() == 204 || playing_res.status() == 202 {
        return get_recently_played(&client, &token_res.access_token).await;
    }

    let track_data: serde_json::Value = match playing_res.json().await {
        Ok(json) => json,
        Err(e) => {
            tracing::error!(error = ?e, "Spotify currently playing JSON parse error");
            return get_recently_played(&client, &token_res.access_token).await;
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

    Json(SpotifyStatus {
        is_playing,
        is_recently_played: !is_playing,
        title,
        artist,
        album_art,
        progress_ms,
        duration_ms,
        track_url,
    })
}
