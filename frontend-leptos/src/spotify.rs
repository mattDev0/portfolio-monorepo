use leptos::prelude::*;
use portfolio_api_types::SpotifyStatus;

#[cfg(target_arch = "wasm32")]
use std::time::Duration;

pub fn format_time(ms: Option<u64>) -> String {
    let ms = match ms {
        Some(m) if m > 0 => m,
        _ => return "0:00".to_string(),
    };
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    if seconds < 10 {
        format!("{}:0{}", minutes, seconds)
    } else {
        format!("{}:{}", minutes, seconds)
    }
}

pub fn calculate_progress_percent(local_ms: u64, duration_ms: u64) -> f32 {
    if duration_ms == 0 {
        0.0
    } else {
        (local_ms as f32 / duration_ms as f32) * 100.0
    }
}

pub fn get_spotify_state_label(is_playing: bool, is_recently_played: bool) -> &'static str {
    if is_playing {
        "Now Playing"
    } else if is_recently_played {
        "Last Played"
    } else {
        "Offline"
    }
}

pub fn get_spotify_tooltip(is_playing: bool, is_recently_played: bool) -> &'static str {
    if is_playing {
        "Live playback"
    } else if is_recently_played {
        "Last played song"
    } else {
        "Offline"
    }
}

pub fn tick_progress_capped(current_ms: u64, duration_ms: u64, delta_ms: u64) -> u64 {
    let next = current_ms + delta_ms;
    if duration_ms > 0 && next > duration_ms {
        duration_ms
    } else {
        next
    }
}

#[derive(Clone, Copy)]
pub struct SpotifySignals {
    pub spotify_data: ReadSignal<Option<SpotifyStatus>>,
    pub local_progress_ms: ReadSignal<u64>,
}

// The signal setters are only used in the browser (wasm32) code below.
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
pub fn use_spotify(is_visible: Signal<bool>) -> SpotifySignals {
    let (spotify_data, set_spotify_data) = signal(None::<SpotifyStatus>);
    let (local_progress_ms, set_local_progress_ms) = signal(0u64);

    #[cfg(target_arch = "wasm32")]
    {
        use crate::api::api_url;
        use gloo_net::http::Request;
        use leptos::prelude::set_interval_with_handle;

        // Fetch Spotify API every 10s (only when visible)
        Effect::new(move |_| {
            if !is_visible.get() {
                return;
            }

            let fetch_spotify = move || {
                leptos::task::spawn_local(async move {
                    let url = api_url("rust", "/api/spotify");
                    match Request::get(&url).send().await {
                        Ok(resp) => {
                            if let Ok(data) = resp.json::<SpotifyStatus>().await {
                                set_local_progress_ms.set(data.progress_ms);
                                set_spotify_data.set(Some(data));
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_2(
                                &"Error fetching Spotify API:".into(),
                                &e.to_string().into(),
                            );
                        }
                    }
                });
            };
            fetch_spotify();

            let fetch_interval =
                set_interval_with_handle(fetch_spotify, Duration::from_millis(10000)).ok();

            // Progress tick every 1s while playing
            let tick_interval = set_interval_with_handle(
                move || {
                    if let Some(data) = spotify_data.get_untracked() {
                        if data.is_playing && data.duration_ms > 0 {
                            set_local_progress_ms.update(|prev| {
                                *prev = tick_progress_capped(*prev, data.duration_ms, 1000);
                            });
                        }
                    }
                },
                Duration::from_millis(1000),
            )
            .ok();

            on_cleanup(move || {
                if let Some(handle) = fetch_interval {
                    handle.clear();
                }
                if let Some(handle) = tick_interval {
                    handle.clear();
                }
            });
        });
    }

    SpotifySignals {
        spotify_data,
        local_progress_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(None), "0:00");
        assert_eq!(format_time(Some(0)), "0:00");
        assert_eq!(format_time(Some(61000)), "1:01");
        assert_eq!(format_time(Some(600000)), "10:00");
        assert_eq!(format_time(Some(90000)), "1:30");
        assert_eq!(format_time(Some(180000)), "3:00");
    }

    #[test]
    fn test_calculate_progress_percent() {
        assert_eq!(calculate_progress_percent(90000, 180000), 50.0);
        assert_eq!(calculate_progress_percent(0, 180000), 0.0);
        assert_eq!(calculate_progress_percent(180000, 180000), 100.0);
        assert_eq!(calculate_progress_percent(100, 0), 0.0);
    }

    #[test]
    fn test_state_labels() {
        assert_eq!(get_spotify_state_label(true, false), "Now Playing");
        assert_eq!(get_spotify_state_label(false, true), "Last Played");
        assert_eq!(get_spotify_state_label(false, false), "Offline");
    }

    #[test]
    fn test_tick_progress_capped() {
        assert_eq!(tick_progress_capped(5000, 10000, 1000), 6000);
        assert_eq!(tick_progress_capped(9500, 10000, 1000), 10000);
        assert_eq!(tick_progress_capped(10000, 10000, 1000), 10000);
    }
}
