use crate::components::{Icon, ThemeToggle};
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn Navigation(
    is_scrolled: Signal<bool>,
    active_section: Signal<&'static str>,
    config: &'static PortfolioConfig,
) -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    let nav_class = move || {
        if is_scrolled.get() {
            "fixed top-0 left-0 right-0 z-40 border-b transition-colors duration-200 bg-[var(--color-nav-bg-scrolled)] backdrop-blur-sm border-[var(--color-nav-border-scrolled)] py-3.5"
        } else {
            "fixed top-0 left-0 right-0 z-40 border-b transition-colors duration-200 bg-transparent border-transparent py-5"
        }
    };

    let nav_style = move || {
        if is_scrolled.get() {
            "box-shadow: var(--color-nav-shadow)"
        } else {
            ""
        }
    };

    let sections = [
        "about",
        "skills",
        "projects",
        "experience",
        "infrastructure",
        "contact",
    ];

    view! {
        <nav class=nav_class style=nav_style>
            <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 flex items-center justify-between">
                <a
                    href="#"
                    class="font-semibold text-base text-[var(--color-text-primary)] hover:text-[var(--color-accent)] transition-colors tracking-tight"
                >
                    {config.name.as_str()}
                </a>
                <div class="hidden md:flex items-center space-x-6">
                    {sections.into_iter().map(|sec| {
                        let is_active = move || active_section.get() == sec;
                        let link_class = move || {
                            if is_active() {
                                "text-xs font-medium capitalize tracking-wide transition-colors duration-200 text-[var(--color-text-primary)] border-b-2 border-[var(--color-accent)] pb-0.5"
                            } else {
                                "text-xs font-medium capitalize tracking-wide transition-colors duration-200 text-[var(--color-text-tertiary)] hover:text-[var(--color-text-primary)]"
                            }
                        };
                        view! {
                            <a href=format!("#{}", sec) class=link_class>
                                {sec}
                            </a>
                        }
                    }).collect_view()}
                    <div class="border-l border-[var(--color-border-primary)] pl-4 ml-1">
                        <ThemeToggle />
                    </div>
                </div>
                // Mobile controls
                <div class="md:hidden flex items-center space-x-2.5">
                    <div class="flex items-center space-x-1.5 bg-[var(--color-bg-surface)] px-2.5 py-1 rounded-md border border-[var(--color-border-primary)] text-[11px] text-[var(--color-text-secondary)] font-mono capitalize">
                        <span class="h-1.5 w-1.5 rounded-full bg-[var(--color-accent)]"></span>
                        <span>{move || active_section.get()}</span>
                    </div>
                    <ThemeToggle />
                    <button
                        type="button"
                        on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                        class="w-9 h-9 rounded-lg border border-[var(--color-border-primary)] bg-[var(--color-hover-bg)] hover:bg-[var(--color-hover-bg-strong)] text-[var(--color-text-tertiary)] hover:text-[var(--color-accent)] hover:border-[var(--color-border-secondary)] transition-colors duration-200 cursor-pointer flex items-center justify-center focus-visible:outline-2 focus-visible:outline-[var(--color-focus-ring)] focus-visible:outline-offset-2"
                        aria-label=move || {
                            if mobile_menu_open.get() {
                                "Close navigation menu"
                            } else {
                                "Open navigation menu"
                            }
                        }
                        aria-expanded=move || mobile_menu_open.get().to_string()
                        aria-controls="mobile-nav-menu"
                    >
                        {move || {
                            if mobile_menu_open.get() {
                                view! { <Icon name="x" class="w-4 h-4" /> }.into_any()
                            } else {
                                view! { <Icon name="menu" class="w-4 h-4" /> }.into_any()
                            }
                        }}
                    </button>
                </div>
            </div>
            {move || {
                if mobile_menu_open.get() {
                    Some(view! {
                        <div
                            id="mobile-nav-menu"
                            class="md:hidden mt-3 mx-4 rounded-xl border border-[var(--color-border-secondary)] bg-[var(--color-bg-modal)] p-2 shadow-lg flex flex-col space-y-1"
                        >
                            {sections.into_iter().map(|sec| {
                                let is_active = move || active_section.get() == sec;
                                let link_class = move || {
                                    if is_active() {
                                        "px-3 py-2 rounded-lg text-xs font-semibold capitalize tracking-wide transition-colors bg-[var(--color-accent-bg)] text-[var(--color-accent)]"
                                    } else {
                                        "px-3 py-2 rounded-lg text-xs font-medium capitalize tracking-wide transition-colors text-[var(--color-text-secondary)] hover:bg-[var(--color-hover-bg-strong)] hover:text-[var(--color-hover-text)]"
                                    }
                                };
                                view! {
                                    <a
                                        href=format!("#{}", sec)
                                        on:click=move |_| set_mobile_menu_open.set(false)
                                        class=link_class
                                    >
                                        {sec}
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    })
                } else {
                    None
                }
            }}
        </nav>
    }
}

#[component]
pub fn Header(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <header class="w-full max-w-5xl mb-12 md:mb-14 pt-4 pb-8 border-b border-[var(--color-border-primary)]">
            <div class="flex flex-col sm:flex-row sm:items-center gap-5 mb-6">
                <img
                    src="/avatar-53d31c1b.webp"
                    width="88"
                    height="88"
                    fetchpriority="high"
                    alt=config.name.as_str()
                    class="w-20 h-20 sm:w-22 sm:h-22 rounded-full border border-[var(--color-border-secondary)] bg-[var(--color-bg-surface)] shrink-0"
                />
                <div>
                    <h1 class="text-3xl sm:text-4xl md:text-5xl font-bold text-[var(--color-text-primary)] tracking-tight leading-tight">
                        {config.name.as_str()}
                    </h1>
                    <h2 class="text-base sm:text-lg font-medium text-[var(--color-text-tertiary)] mt-1">
                        {config.title.as_str()}
                    </h2>
                    <div class="flex flex-wrap items-center gap-x-3 gap-y-1 text-xs sm:text-sm text-[var(--color-text-tertiary)] mt-2.5">
                        <span class="inline-flex items-center">
                            <svg
                                class="w-3.5 h-3.5 mr-1.5 text-[var(--color-text-muted)]"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                                xmlns="http://www.w3.org/2000/svg"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
                                />
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
                                />
                            </svg>
                            <span class="text-[var(--color-text-secondary)]">{config.location.as_str()}</span>
                        </span>

                        {if config.open_to_remote {
                            Some(view! {
                                <span class="text-[var(--color-text-faint)]">"·"</span>
                                <span class="inline-flex items-center gap-1.5 font-mono text-xs text-[var(--color-text-secondary)]">
                                    <span class="h-1.5 w-1.5 rounded-full bg-[var(--color-status-online)]"></span>
                                    <span>"Open to remote"</span>
                                </span>
                            })
                        } else {
                            None
                        }}
                    </div>
                </div>
            </div>

            // Tagline
            <p class="max-w-2xl text-[var(--color-text-secondary)] leading-relaxed text-sm sm:text-base mb-7">
                {config.tagline.as_str()}
            </p>

            // Contact Links
            <div class="grid grid-cols-2 gap-2.5 sm:flex sm:flex-wrap sm:items-center sm:gap-3">
                <a
                    href=config.resume_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center space-x-2 px-4 py-2 bg-[var(--color-accent-solid)] border border-[var(--color-accent-solid)] text-white hover:bg-[var(--color-accent-solid-hover)] rounded-lg transition-colors duration-200 w-full sm:w-auto"
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <span class="font-semibold text-sm">"Resume"</span>
                </a>
                <a
                    href=config.github_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center space-x-2 px-4 py-2 bg-[var(--color-bg-surface)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] rounded-lg transition-colors duration-200 w-full sm:w-auto"
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.464-1.11-1.464-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.579.688.481C19.138 20.161 22 16.418 22 12c0-5.523-4.477-10-10-10z"
                        />
                    </svg>
                    <span class="font-medium text-sm">"GitHub"</span>
                </a>
                <a
                    href=config.linkedin_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center space-x-2 px-4 py-2 bg-[var(--color-bg-surface)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] rounded-lg transition-colors duration-200 w-full sm:w-auto"
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
                    </svg>
                    <span class="font-medium text-sm">"LinkedIn"</span>
                </a>
                <a
                    href=format!("mailto:{}", &config.email)
                    class="flex items-center justify-center space-x-2 px-4 py-2 bg-[var(--color-bg-surface)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] rounded-lg transition-colors duration-200 w-full sm:w-auto"
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                    </svg>
                    <span class="font-medium text-sm">"Email"</span>
                </a>
            </div>
        </header>
    }
}
