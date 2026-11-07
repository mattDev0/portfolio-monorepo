use crate::theme::{Theme, ThemeContext, theme_aria_label, theme_title};
use leptos::prelude::*;

const ICON_ACTIVE_CLASS: &str = "absolute inset-0 flex items-center justify-center transition-all duration-300 opacity-100 transform rotate-0 scale-100";
const ICON_INACTIVE_CLASS: &str = "absolute inset-0 flex items-center justify-center transition-all duration-300 opacity-0 transform -rotate-90 scale-50 pointer-events-none";

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let ctx = use_context::<ThemeContext>().expect("ThemeContext must be provided");

    let on_click = move |_| {
        ctx.cycle();
    };

    let aria_label = move || theme_aria_label(ctx.theme.get());
    let title_text = move || theme_title(ctx.theme.get(), ctx.resolved_theme.get());

    let sun_class = move || {
        if ctx.theme.get() == Theme::Light {
            ICON_ACTIVE_CLASS
        } else {
            ICON_INACTIVE_CLASS
        }
    };

    let moon_class = move || {
        if ctx.theme.get() == Theme::Dark {
            ICON_ACTIVE_CLASS
        } else {
            ICON_INACTIVE_CLASS
        }
    };

    let system_class = move || {
        if ctx.theme.get() == Theme::System {
            ICON_ACTIVE_CLASS
        } else {
            ICON_INACTIVE_CLASS
        }
    };

    view! {
        <button
            on:click=on_click
            class="relative w-9 h-9 rounded-lg border border-[var(--color-border-primary)] bg-[var(--color-hover-bg)] text-[var(--color-text-tertiary)] hover:text-[var(--color-accent-emerald)] hover:border-[var(--color-border-secondary)] transition-all duration-200 cursor-pointer flex items-center justify-center focus-visible:outline-2 focus-visible:outline-[var(--color-focus-ring)] focus-visible:outline-offset-2"
            aria-label=aria_label
            title=title_text
        >
            <div class="relative w-5 h-5 flex items-center justify-center overflow-hidden">
                // Sun Icon
                <span class=sun_class>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="w-5 h-5"
                    >
                        <circle cx="12" cy="12" r="4"></circle>
                        <path d="M12 2v2"></path>
                        <path d="M12 20v2"></path>
                        <path d="M4.93 4.93l1.41 1.41"></path>
                        <path d="M17.66 17.66l1.41 1.41"></path>
                        <path d="M2 12h2"></path>
                        <path d="M20 12h2"></path>
                        <path d="M6.34 17.66l-1.41 1.41"></path>
                        <path d="M19.07 4.93l-1.41 1.41"></path>
                    </svg>
                </span>

                // Moon Icon
                <span class=moon_class>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="w-5 h-5"
                    >
                        <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
                    </svg>
                </span>

                // System Monitor Icon
                <span class=system_class>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="w-5 h-5"
                    >
                        <rect width="20" height="14" x="2" y="3" rx="2"></rect>
                        <path d="M8 21h8"></path>
                        <path d="M12 17v4"></path>
                    </svg>
                </span>
            </div>
        </button>
    }
}
