use crate::spotify::{
    calculate_progress_percent, format_time, get_spotify_state_label, get_spotify_tooltip,
};
use leptos::prelude::*;
use portfolio_api_types::SpotifyStatus;

#[component]
pub fn SpotifyPlayer(
    spotify_data: ReadSignal<Option<SpotifyStatus>>,
    local_progress_ms: ReadSignal<u64>,
) -> impl IntoView {
    let view_content = move || {
        let data_opt = spotify_data.get();
        let is_playing = data_opt.as_ref().map(|d| d.is_playing).unwrap_or(false);
        let is_recently_played = data_opt
            .as_ref()
            .map(|d| d.is_recently_played)
            .unwrap_or(false);

        let ping_bg_class = if is_playing {
            "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 bg-[var(--color-status-online)]"
        } else if is_recently_played {
            "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 bg-[var(--color-accent-amber)]"
        } else {
            "animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 bg-[var(--color-status-muted)]"
        };

        let dot_class = if is_playing {
            "relative inline-flex rounded-full h-3 w-3 bg-[var(--color-status-online)] shadow-[0_0_8px_#10b981]"
        } else if is_recently_played {
            "relative inline-flex rounded-full h-3 w-3 bg-[var(--color-status-warning)] shadow-[0_0_8px_#f59e0b]"
        } else {
            "relative inline-flex rounded-full h-3 w-3 bg-[var(--color-status-muted)]"
        };

        let tooltip_text = get_spotify_tooltip(is_playing, is_recently_played);
        let anim_duration = if is_playing { "6s" } else { "0s" };

        view! {
            <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                <span class="flex h-3 w-3 relative">
                    <span class=ping_bg_class></span>
                    {if is_playing {
                        Some(view! {
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--color-status-online)] opacity-40 delay-300"></span>
                        })
                    } else {
                        None
                    }}
                    <span class=dot_class></span>
                </span>
                // Custom Tooltip
                <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-emerald-border)] p-2 text-center text-[10px] text-[var(--color-accent-emerald)] font-mono shadow-xl z-20">
                    {tooltip_text}
                </div>
            </div>

            <h3 class="text-lg font-bold text-[var(--color-accent-emerald)] mb-0 tracking-wide flex items-center font-sans">
                <svg
                    class="w-5 h-5 mr-2 text-[var(--color-accent-emerald)] group-hover:animate-spin"
                    style=format!("animation-duration: {}", anim_duration)
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z" />
                </svg>
                "Spotify Session"
            </h3>
            <p class="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">
                "Web API sync via Rust token auth"
            </p>

            {match data_opt {
                Some(data) => {
                    let album_art = data.album_art.clone();
                    let title = data.title.clone();
                    let artist = data.artist.clone();
                    let duration_ms = data.duration_ms;
                    let local_ms = local_progress_ms.get();
                    let percent = calculate_progress_percent(local_ms, duration_ms);

                    let art_box_class = if is_playing {
                        "w-16 h-16 bg-[var(--color-bg-tag)] rounded-lg flex-shrink-0 shadow-md overflow-hidden relative border border-[var(--color-border-primary)] transition-transform duration-500 group-hover:rotate-12 group-hover:scale-105"
                    } else {
                        "w-16 h-16 bg-[var(--color-bg-tag)] rounded-lg flex-shrink-0 shadow-md overflow-hidden relative border border-[var(--color-border-primary)] transition-transform duration-500"
                    };

                    let art_img_class = if is_playing {
                        "w-full h-full object-cover transition-all duration-500"
                    } else {
                        "w-full h-full object-cover transition-all duration-500 grayscale opacity-60"
                    };

                    let state_text_class = if is_playing {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-accent-emerald)]"
                    } else if is_recently_played {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-accent-amber)]"
                    } else {
                        "text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold text-[var(--color-text-muted)]"
                    };

                    let state_label = get_spotify_state_label(is_playing, is_recently_played);

                    view! {
                        <div>
                            <div class="flex items-center space-x-4 mt-2">
                                <div class=art_box_class>
                                    {if !album_art.is_empty() {
                                        view! {
                                            <img src=album_art alt="Album Art" class=art_img_class />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="absolute inset-0 bg-gradient-to-tr from-[var(--color-accent-emerald-solid)] to-[var(--color-bg-primary)] opacity-50"></div>
                                        }.into_any()
                                    }}
                                </div>
                                <div class="overflow-hidden flex-grow">
                                    <p class="text-[var(--color-text-primary)] font-bold truncate group-hover:text-[var(--color-accent-emerald)] transition-colors">
                                        {title}
                                    </p>
                                    <p class="text-[var(--color-text-tertiary)] text-sm truncate">
                                        {artist}
                                    </p>
                                    <p class=state_text_class>
                                        {state_label}
                                    </p>
                                </div>
                            </div>

                            {if is_playing && duration_ms > 0 {
                                Some(view! {
                                    <div class="mt-5">
                                        <div class="w-full h-1 bg-[var(--color-bg-progress-track)] rounded-full overflow-hidden">
                                            <div
                                                class="h-full bg-[var(--color-accent-emerald-solid)] transition-all duration-1000 ease-linear shadow-[0_0_8px_var(--color-accent-emerald)]"
                                                style=format!("width: {}%", percent)
                                            ></div>
                                        </div>
                                        <div class="flex justify-between text-[9px] text-[var(--color-text-muted)] mt-1.5 font-mono">
                                            <span>{format_time(Some(local_ms))}</span>
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
                None => {
                    view! {
                        <p class="text-[var(--color-text-muted)] animate-pulse font-mono text-xs mt-2">
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
                        class="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-emerald-border)] hover:border-[var(--color-accent-emerald)] relative overflow-hidden transition-all duration-300 group flex flex-col justify-center cursor-pointer"
                        style="box-shadow: var(--shadow-card)"
                    >
                        {view_content()}
                    </a>
                }.into_any()
            } else {
                view! {
                    <section
                        class="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] relative overflow-hidden transition-all duration-300 flex flex-col justify-center"
                        style="box-shadow: var(--shadow-card)"
                    >
                        {view_content()}
                    </section>
                }.into_any()
            }
        }}
    }
}
