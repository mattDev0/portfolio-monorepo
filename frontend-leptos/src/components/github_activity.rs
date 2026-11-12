use leptos::prelude::*;
use portfolio_api_types::GitHubCommitActivity;

pub fn format_commit_date(date_str: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let d = js_sys::Date::new(&wasm_bindgen::JsValue::from_str(date_str));
        let opts = js_sys::Object::new();
        let _ = js_sys::Reflect::set(&opts, &"month".into(), &"short".into());
        let _ = js_sys::Reflect::set(&opts, &"day".into(), &"numeric".into());
        let _ = js_sys::Reflect::set(&opts, &"year".into(), &"numeric".into());
        d.to_locale_date_string("en-US", &opts)
            .as_string()
            .unwrap_or_else(|| date_str.to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        date_str.to_string()
    }
}

// The signal setters are only used in the browser (wasm32) code below.
#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
#[component]
pub fn GitHubActivity() -> impl IntoView {
    let (commits, set_commits) = signal(Vec::<GitHubCommitActivity>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<&'static str>);

    #[cfg(target_arch = "wasm32")]
    {
        use crate::api::{api_url, fetch_json};

        const GITHUB_TIMEOUT_MS: u32 = 8_000;

        Effect::new(move |_| {
            leptos::task::spawn_local(async move {
                let url = api_url("java", "/api/github/activity");
                match fetch_json::<Vec<GitHubCommitActivity>>(&url, GITHUB_TIMEOUT_MS).await {
                    Ok(data) => {
                        set_commits.set(data);
                        set_loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::error_2(&"GitHub fetch error:".into(), &e.into());
                        set_error.set(Some("Activity feed temporarily unavailable."));
                        set_loading.set(false);
                    }
                }
            });
        });
    }

    view! {
        <div class="card-surface p-6 mb-8 w-full transition-colors duration-300">
            <div class="flex items-center mb-6">
                <svg
                    class="w-6 h-6 text-[var(--color-accent)] mr-3 animate-pulse drop-shadow-[0_0_8px_var(--color-accent-border)]"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                    />
                </svg>
                <h3 class="text-xl font-bold text-[var(--color-text-primary)] tracking-wide">
                    "Recent GitHub Commits (Java Infra)"
                </h3>
            </div>

            {move || {
                if loading.get() {
                    view! {
                        <div class="animate-pulse space-y-4">
                            <div class="h-16 bg-[var(--color-bg-tag)] border border-[var(--color-border-primary)] rounded-xl"></div>
                            <div class="h-16 bg-[var(--color-bg-tag)] border border-[var(--color-border-primary)] rounded-xl"></div>
                            <div class="h-16 bg-[var(--color-bg-tag)] border border-[var(--color-border-primary)] rounded-xl"></div>
                            <div class="h-16 bg-[var(--color-bg-tag)] border border-[var(--color-border-primary)] rounded-xl"></div>
                        </div>
                    }.into_any()
                } else if let Some(err) = error.get() {
                    view! {
                        <p class="text-[var(--color-text-tertiary)] italic text-sm">{err}</p>
                    }.into_any()
                } else {
                    let commit_items = commits.get();
                    view! {
                        <ul class="space-y-4">
                            {commit_items.into_iter().map(|commit| {
                                let formatted_date = format_commit_date(&commit.date);
                                view! {
                                    <li class="pb-4 border-b border-[var(--color-border-primary)] last:border-0 last:pb-0 group/item">
                                        // Repo Name & Date
                                        <div class="flex justify-between items-center mb-1.5">
                                            <span class="text-[var(--color-accent)] font-semibold text-sm truncate pr-4 group-hover/item:text-[var(--color-accent-solid-hover)] transition-colors">
                                                {commit.repo}
                                            </span>
                                            <span class="text-[var(--color-text-muted)] text-xs font-medium">
                                                {formatted_date}
                                            </span>
                                        </div>

                                        // Hash Badge & Commit Message
                                        <div class="flex items-center space-x-2.5">
                                            <span class="bg-[var(--color-bg-tag)] border border-[var(--color-accent-border)] text-[var(--color-accent)] font-mono text-[10px] px-2 py-0.5 rounded shadow-sm tracking-wider">
                                                {commit.hash}
                                            </span>
                                            <span class="text-[var(--color-text-secondary)] text-sm truncate group-hover/item:text-[var(--color-text-primary)] transition-colors">
                                                {commit.message}
                                            </span>
                                        </div>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_any()
                }
            }}
        </div>
    }
}
