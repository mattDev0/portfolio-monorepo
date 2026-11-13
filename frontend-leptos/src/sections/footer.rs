use crate::config::PortfolioConfig;
use leptos::prelude::*;

pub fn get_current_year() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::new_0().get_full_year()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        2026
    }
}

#[component]
pub fn Footer(config: &'static PortfolioConfig) -> impl IntoView {
    let year = get_current_year();

    view! {
        <footer class="w-full max-w-5xl mt-16 pt-8 pb-4 border-t border-[var(--color-border-primary)] flex flex-col md:flex-row items-center justify-between gap-4 text-xs text-[var(--color-text-muted)]">
            <div class="flex items-center space-x-2">
                <span>{format!("© {} {}. All rights reserved.", year, &config.name)}</span>
            </div>
            <div class="flex items-center space-x-1 text-[var(--color-text-muted)]">
                <span>"Architected with"</span>
                <span class="text-[var(--color-accent-orange)] font-semibold font-mono">"Rust"</span>
                <span>"•"</span>
                <span class="text-[var(--color-accent)] font-semibold font-mono">"Spring Boot"</span>
                <span>"•"</span>
                <span class="text-[var(--color-accent-gold)] font-semibold font-mono">"Leptos"</span>
            </div>
            <div class="flex items-center space-x-4">
                <a
                    href=config.github_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:text-[var(--color-hover-text)] transition-colors duration-200"
                    title="GitHub"
                >
                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path
                            fill-rule="evenodd"
                            clip-rule="evenodd"
                            d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.464-1.11-1.464-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.579.688.481C19.138 20.161 22 16.418 22 12c0-5.523-4.477-10-10-10z"
                        />
                    </svg>
                </a>
                <a
                    href=config.linkedin_url.as_str()
                    target="_blank"
                    rel="noopener noreferrer"
                    class="hover:text-[var(--color-hover-text)] transition-colors duration-200"
                    title="LinkedIn"
                >
                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.064 2.064 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" />
                    </svg>
                </a>
                <a
                    href=format!("mailto:{}", &config.email)
                    class="hover:text-[var(--color-hover-text)] transition-colors duration-200"
                    title="Email"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                    </svg>
                </a>
            </div>
        </footer>
    }
}
