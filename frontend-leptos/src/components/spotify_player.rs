use crate::components::Icon;
use crate::feed::{FeedHealth, FeedState, format_age, now_ms};
use crate::spotify::{
    calculate_progress_percent, format_time, get_spotify_state_label, get_spotify_tooltip,
};
use leptos::prelude::*;
use portfolio_api_types::SpotifyStatus;

#[component]
pub fn SpotifyPlayer(
    spotify_data: ReadSignal<Option<SpotifyStatus>>,
    local_progress_ms: ReadSignal<u64>,
    spotify_health: ReadSignal<FeedHealth>,
) -> impl IntoView {
    let view_content = move || {
        let data_opt = spotify_data.get();
        let feed_state = spotify_health.get().state(now_ms());
        let is_stale = matches!(feed_state, FeedState::Stale { .. });
        // Stale data must not look live: no "Now Playing", no ticking progress.
        let is_playing = !is_stale && data_opt.as_ref().map(|d| d.is_playing).unwrap_or(false);
        let is_recently_played = !is_stale
            && data_opt
                .as_ref()
                .map(|d| d.is_recently_played)
                .unwrap_or(false);

        let dot_class = if is_playing {
            "inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-status-online)]"
        } else if is_recently_played || is_stale {
            "inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-status-warning)]"
        } else {
            "inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-status-muted)]"
        };

        let tooltip_text = get_spotify_tooltip(is_playing, is_recently_played, is_stale);

        view! {
            <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                <span class=dot_class></span>
                // Custom Tooltip
                <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-transform duration-150 origin-top-right rounded-md bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-secondary)] p-2 text-center text-[10px] text-[var(--color-text-primary)] font-mono shadow-md z-20">
                    {tooltip_text}
                </div>
            </div>

            <h3 class="text-base font-bold text-[var(--color-text-primary)] mb-0.5 tracking-tight flex items-center font-sans">
                <svg
                    class="w-4 h-4 mr-2 text-[var(--color-accent)] shrink-0"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z" />
                </svg>
                "Spotify Session"
            </h3>
            <p class="text-[11px] text-[var(--color-text-muted)] font-sans mb-4">
                "Web API sync via Rust token auth"
            </p>

            {match data_opt {
                Some(data) => {
                    let album_art = data.album_art.clone();
                    let title = data.title.clone();
                    let artist = data.artist.clone();
                    let duration_ms = data.duration_ms;

                    let art_img_class = if is_playing {
                        "w-full h-full object-cover"
                    } else {
                        "w-full h-full object-cover grayscale opacity-65"
                    };

                    let state_text_class = if is_stale {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-status-warning)]"
                    } else if is_playing {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-status-online)]"
                    } else if is_recently_played {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-accent-amber)]"
                    } else {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-text-muted)]"
                    };

                    let state_label = get_spotify_state_label(is_playing, is_recently_played, is_stale);
                    let stale_age = match feed_state {
                        FeedState::Stale { age_secs } => Some(format!("Updated {}", format_age(age_secs))),
                        _ => None,
                    };

                    view! {
                        <div>
                            <div class="flex items-center space-x-3.5 mt-1">
                                <div class="w-14 h-14 bg-[var(--color-bg-tag)] rounded-md flex-shrink-0 overflow-hidden relative border border-[var(--color-border-primary)] flex items-center justify-center">
                                    {if !album_art.is_empty() {
                                        view! {
                                            <img src=album_art alt="Album Art" class=art_img_class />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <Icon name="music" class="w-5 h-5 text-[var(--color-text-muted)]" />
                                        }.into_any()
                                    }}
                                </div>
                                <div class="overflow-hidden flex-grow">
                                    <p class="text-[var(--color-text-primary)] font-sans font-semibold text-sm truncate group-hover:text-[var(--color-accent)] transition-colors">
                                        {title}
                                    </p>
                                    <p class="text-[var(--color-text-tertiary)] font-sans text-xs truncate">
                                        {artist}
                                    </p>
                                    <p class=state_text_class>
                                        {state_label}
                                    </p>
                                    {stale_age.map(|age| view! {
                                        <p class="text-[10px] font-mono text-[var(--color-text-muted)]">{age}</p>
                                    })}
                                </div>
                            </div>

                            {if is_playing && duration_ms > 0 {
                                Some(view! {
                                    <div class="mt-4">
                                        <div class="w-full h-1 bg-[var(--color-bg-progress-track)] rounded-full overflow-hidden">
                                            <div
                                                class="h-full bg-[var(--color-accent-solid)] transition-all duration-1000 ease-linear"
                                                style=move || format!("width: {}%", calculate_progress_percent(local_progress_ms.get(), duration_ms))
                                            ></div>
                                        </div>
                                        <div class="flex justify-between text-[10px] text-[var(--color-text-muted)] mt-1.5 font-mono">
                                            <span>{move || format_time(Some(local_progress_ms.get()))}</span>
                                            <span>{format_time(Some(duration_ms))}</span>
                                        </div>
                                    </div>
                                })
                            } else {
                                None
                            }}
                        </div>
                    }.into_any()
                }
                None if feed_state == FeedState::Unavailable => {
                    view! {
                        <p class="text-[var(--color-text-muted)] font-mono text-xs mt-2">
                            "Spotify feed unavailable"
                        </p>
                    }.into_any()
                }
                None => {
                    view! {
                        <p class="text-[var(--color-text-muted)] font-mono text-xs mt-2">
                            "Connecting to Spotify Web API..."
                        </p>
                    }.into_any()
                }
            }}
        }
    };

    view! {
        {move || {
            let data = spotify_data.get();
            let has_track_url = data.as_ref().map(|d| !d.track_url.is_empty()).unwrap_or(false);
            if has_track_url {
                let track_url = data.unwrap().track_url;
                view! {
                    <a
                        href=track_url
                        target="_blank"
                        rel="noopener noreferrer"
                        class="card-surface hover:border-[var(--color-border-secondary)] p-5 rounded-xl relative group flex flex-col justify-center cursor-pointer"
                    >
                        {view_content()}
                    </a>
                }.into_any()
            } else {
                view! {
                    <section class="card-surface p-5 rounded-xl relative flex flex-col justify-center">
                        {view_content()}
                    </section>
                }.into_any()
            }
        }}
    }
}
