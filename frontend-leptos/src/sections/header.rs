use crate::components::ThemeToggle;
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn Navigation(
    is_scrolled: Signal<bool>,
    active_section: Signal<&'static str>,
    config: &'static PortfolioConfig,
) -> impl IntoView {
    let nav_class = move || {
        if is_scrolled.get() {
            "fixed top-0 left-0 right-0 z-40 border-b transition-all duration-300 bg-[var(--color-nav-bg-scrolled)] backdrop-blur-md border-[var(--color-nav-border-scrolled)] py-4"
        } else {
            "fixed top-0 left-0 right-0 z-40 border-b transition-all duration-300 bg-transparent border-transparent py-6"
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
    ];

    view! {
        <nav class=nav_class style=nav_style>
            <div class="max-w-5xl mx-auto px-4 sm:px-8 flex items-center justify-between">
                <a
                    href="#"
                    class="font-bold text-lg bg-clip-text text-transparent tracking-tight"
                    style="background-image: var(--gradient-name)"
                >
                    {config.name.as_str()}
                </a>
                <div class="hidden md:flex items-center space-x-6">
                    {sections.into_iter().map(|sec| {
                        let is_active = move || active_section.get() == sec;
                        let link_class = move || {
                            if is_active() {
                                "text-xs font-bold tracking-wide uppercase transition-all duration-300 hover:text-[var(--color-hover-text)] hover:scale-105 text-[var(--color-accent)] border-b border-[var(--color-accent)] pb-1"
                            } else {
                                "text-xs font-bold tracking-wide uppercase transition-all duration-300 hover:text-[var(--color-hover-text)] hover:scale-105 text-[var(--color-text-tertiary)]"
                            }
                        };
                        view! {
                            <a href=format!("#{}", sec) class=link_class>
                                {sec}
                            </a>
                        }
                    }).collect_view()}
                    <div class="border-l border-[var(--color-border-primary)] pl-4 ml-2">
                        <ThemeToggle />
                    </div>
                </div>
                // Subtle Mobile indicators
                <div class="md:hidden flex items-center space-x-3">
                    <div class="flex items-center space-x-2 bg-[var(--color-bg-surface)] px-3 py-1 rounded-full border border-[var(--color-border-primary)] text-[10px] text-[var(--color-accent)] font-bold uppercase tracking-wider font-mono">
                        <span class="h-1.5 w-1.5 rounded-full bg-[var(--color-status-online)] animate-ping"></span>
                        <span>{move || active_section.get()}</span>
                    </div>
                    <ThemeToggle />
                </div>
            </div>
        </nav>
    }
}

#[component]
pub fn Header(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <header class="text-center mb-10 md:mb-12 max-w-3xl w-full">
            // Avatar / Profile Picture
            <div class="mb-6 relative inline-block group">
                <div
                    class="absolute inset-0 rounded-full blur opacity-40 group-hover:opacity-75 transition duration-500"
                    style="background-image: var(--gradient-glow)"
                ></div>
                <img
                    src="https://github.com/mattDev0.png"
                    alt=config.name.as_str()
                    class="relative w-24 h-24 sm:w-28 sm:h-28 rounded-full border-2 border-[var(--color-accent-border)] p-1 bg-[var(--color-bg-primary)] shadow-2xl transition-transform duration-500 group-hover:scale-105"
                />
            </div>

            <h1
                class="text-4xl sm:text-5xl md:text-6xl font-extrabold bg-clip-text text-transparent mb-4 tracking-tight"
                style="background-image: var(--gradient-name)"
            >
                {config.name.as_str()}
            </h1>
            <h2 class="text-lg sm:text-xl font-medium text-[var(--color-text-tertiary)] mb-4 tracking-wide">
                {config.title.as_str()}
            </h2>

            // Location and Remote Status
            <div class="flex items-center justify-center text-[var(--color-text-tertiary)] mb-6 text-sm">
                <svg
                    class="w-4 h-4 mr-2 text-[var(--color-accent)]"
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
                <span class="font-medium text-[var(--color-text-secondary)]">{config.location.as_str()}</span>

                {if config.open_to_remote {
                    Some(view! {
                        <span class="mx-3 text-[var(--color-text-faint)]">"•"</span>
                        <span class="bg-[var(--color-accent-bg)] text-[var(--color-accent)] border border-[var(--color-accent-border)] px-2 py-0.5 rounded-full text-xs font-semibold tracking-wider uppercase">
                            "Open to Remote"
                        </span>
                    })
                } else {
                    None
                }}
            </div>

            // Tagline
            <p class="max-w-2xl mx-auto text-[var(--color-text-secondary)] leading-relaxed text-sm sm:text-base mb-8">
                {config.tagline.as_str()}
            </p>

            // Contact Links
            <div class="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 max-w-sm sm:max-w-none mx-auto w-full sm:w-auto flex-wrap">
                <a
                    href=config.github_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center space-x-2 px-6 py-2.5 bg-[var(--color-accent-solid)] border border-[var(--color-accent-solid)] text-white hover:bg-[var(--color-accent-solid-hover)] rounded-xl transition-all duration-300 w-full sm:w-auto"
                    style="box-shadow: var(--shadow-button)"
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.464-1.11-1.464-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.579.688.481C19.138 20.161 22 16.418 22 12c0-5.523-4.477-10-10-10z"
                        />
                    </svg>
                    <span class="font-semibold text-sm">"GitHub"</span>
                </a>
                <a
                    href=config.linkedin_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center space-x-2 px-6 py-2.5 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] rounded-xl transition-all duration-300 w-full sm:w-auto"
                >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
                    </svg>
                    <span class="font-semibold text-sm">"LinkedIn"</span>
                </a>
                <a
                    href=format!("mailto:{}", &config.email)
                    class="flex items-center justify-center space-x-2 px-6 py-2.5 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] rounded-xl transition-all duration-300 w-full sm:w-auto"
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                    </svg>
                    <span class="font-semibold text-sm">"Email Me"</span>
                </a>
                <a
                    href=config.resume_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="flex items-center justify-center space-x-2 px-6 py-2.5 bg-[var(--color-accent-solid)] border border-[var(--color-accent-solid)] text-white hover:bg-[var(--color-accent-solid-hover)] rounded-xl transition-all duration-300 w-full sm:w-auto"
                    style="box-shadow: var(--shadow-button)"
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                    </svg>
                    <span class="font-semibold text-sm">"Resume"</span>
                </a>
            </div>
        </header>
    }
}
