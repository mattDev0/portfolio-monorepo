use crate::feed::{FeedHealth, FeedState, format_age, now_ms};
use leptos::prelude::*;

/// Accent used by a card's status dot while its feed is live.
#[derive(Clone, Copy, PartialEq)]
pub enum DotAccent {
    Orange,
    Online,
}

/// Corner status dot: solid accent while live or connecting, static amber when
/// the data is stale and static grey when the feed has never answered.
#[component]
pub fn FeedDot(health: ReadSignal<FeedHealth>, accent: DotAccent) -> impl IntoView {
    move || {
        match health.get().state(now_ms()) {
        FeedState::Live | FeedState::Connecting => match accent {
            DotAccent::Orange => view! {
                <span class="inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-accent-orange)]"></span>
            }
            .into_any(),
            DotAccent::Online => view! {
                <span class="inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-status-online)]"></span>
            }
            .into_any(),
        },
        FeedState::Stale { .. } => view! {
            <span class="inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-status-warning)]"></span>
        }
        .into_any(),
        FeedState::Unavailable => view! {
            <span class="inline-flex rounded-full h-2.5 w-2.5 bg-[var(--color-status-muted)]"></span>
        }
        .into_any(),
    }
    }
}

/// One-line notice shown above a card's data while that data is stale.
#[component]
pub fn StaleNotice(health: ReadSignal<FeedHealth>) -> impl IntoView {
    move || match health.get().state(now_ms()) {
        FeedState::Stale { age_secs } => Some(view! {
            <p class="text-[10px] font-mono text-[var(--color-status-warning)] mb-2">
                {format!("Stale data · last updated {}", format_age(age_secs))}
            </p>
        }),
        _ => None,
    }
}

/// Placeholder before the first successful poll.
#[component]
pub fn FeedPending(health: ReadSignal<FeedHealth>, label: &'static str) -> impl IntoView {
    move || {
        if health.get().state(now_ms()) == FeedState::Unavailable {
            view! {
                <p class="text-[var(--color-text-muted)] font-mono text-xs mt-2">
                    "Service unreachable"
                </p>
            }
            .into_any()
        } else {
            view! {
                <p class="text-[var(--color-text-muted)] font-mono text-xs mt-2">
                    {label}
                </p>
            }
            .into_any()
        }
    }
}
