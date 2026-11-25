use crate::components::{Icon, SectionHeading};
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyState {
    Idle,
    Copied,
    Failed,
}

pub fn copy_button_label(state: CopyState) -> &'static str {
    match state {
        CopyState::Idle => "Copy",
        CopyState::Copied => "Copied",
        CopyState::Failed => "Select to copy",
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
#[component]
pub fn Contact(config: &'static PortfolioConfig) -> impl IntoView {
    let (copy_state, set_copy_state) = signal(CopyState::Idle);
    let reset_timer = StoredValue::new(None::<TimeoutHandle>);

    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        if let Some(handle) = reset_timer.try_get_value().flatten() {
            handle.clear();
        }
    });

    let email = config.email.as_str();
    let copy_email = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use std::time::Duration;
            use wasm_bindgen::{JsValue, closure::Closure};

            let schedule_reset = move || {
                if let Some(prev) = reset_timer.try_get_value().flatten() {
                    prev.clear();
                }
                if let Ok(handle) = set_timeout_with_handle(
                    move || set_copy_state.set(CopyState::Idle),
                    Duration::from_secs(2),
                ) {
                    reset_timer.set_value(Some(handle));
                }
            };

            if let Some(window) = web_sys::window() {
                let promise = window.navigator().clipboard().write_text(email);
                let on_resolve = Closure::once(move |_: JsValue| {
                    set_copy_state.set(CopyState::Copied);
                    schedule_reset();
                });
                let on_reject = Closure::once(move |_: JsValue| {
                    set_copy_state.set(CopyState::Failed);
                    schedule_reset();
                });
                let _ = promise.then2(&on_resolve, &on_reject);
                on_resolve.forget();
                on_reject.forget();
            } else {
                set_copy_state.set(CopyState::Failed);
                schedule_reset();
            }
        }
    };

    let copy_btn_class = move || match copy_state.get() {
        CopyState::Idle => {
            "shrink-0 inline-flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold border border-[var(--color-border-primary)] bg-[var(--color-hover-bg)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] transition-colors cursor-pointer"
        }
        CopyState::Copied => {
            "shrink-0 inline-flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold border border-[var(--color-status-online)]/40 bg-[var(--color-bg-surface)] text-[var(--color-status-online)] transition-colors cursor-pointer"
        }
        CopyState::Failed => {
            "shrink-0 inline-flex items-center space-x-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold border border-[var(--color-status-warning)]/40 bg-[var(--color-bg-surface)] text-[var(--color-status-warning)] transition-colors cursor-pointer"
        }
    };

    view! {
        <section
            id="contact"
            class="reveal-in card-surface p-6 md:p-8 rounded-xl"
        >
            <SectionHeading eyebrow="Reach Out" title="Get in Touch" />

            <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-6">
                <div class="space-y-2 max-w-xl">
                    <p class="text-[var(--color-text-secondary)] text-sm sm:text-base leading-relaxed">
                        "Open to DevOps, Platform, SRE, and Backend Engineering roles — remote worldwide or on-site in Lagos, Nigeria."
                    </p>
                    <p class="text-[var(--color-text-tertiary)] text-xs sm:text-sm leading-relaxed">
                        "The fastest way to reach me is directly by email or on LinkedIn."
                    </p>
                </div>

                <div class="flex flex-col gap-3 w-full lg:w-auto lg:min-w-[360px]">
                    <div class="flex items-center justify-between gap-3 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] rounded-lg px-4 py-2.5">
                        <div class="flex items-center space-x-2.5 min-w-0">
                            <Icon name="mail" class="w-4 h-4 text-[var(--color-accent)]" />
                            <span class="font-mono text-xs sm:text-sm text-[var(--color-text-primary)] select-all truncate">
                                {email}
                            </span>
                        </div>
                        <button
                            type="button"
                            on:click=copy_email
                            class=copy_btn_class
                            aria-live="polite"
                        >
                            {move || match copy_state.get() {
                                CopyState::Idle => view! {
                                    <Icon name="copy" class="w-3.5 h-3.5" />
                                    <span>{copy_button_label(CopyState::Idle)}</span>
                                }.into_any(),
                                CopyState::Copied => view! {
                                    <Icon name="check" class="w-3.5 h-3.5" />
                                    <span>{copy_button_label(CopyState::Copied)}</span>
                                }.into_any(),
                                CopyState::Failed => view! {
                                    <Icon name="copy" class="w-3.5 h-3.5" />
                                    <span>{copy_button_label(CopyState::Failed)}</span>
                                }.into_any(),
                            }}
                        </button>
                    </div>

                    <div class="grid grid-cols-2 sm:flex sm:flex-wrap gap-2.5">
                        <a
                            href=format!("mailto:{}", email)
                            class="inline-flex items-center justify-center space-x-1.5 px-4 py-2 bg-[var(--color-accent-solid)] hover:bg-[var(--color-accent-solid-hover)] text-white rounded-lg text-xs font-semibold transition-colors duration-200"
                        >
                            <Icon name="mail" class="w-3.5 h-3.5" />
                            <span>"Email Me"</span>
                        </a>
                        <a
                            href=config.linkedin_url.as_str()
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center justify-center px-4 py-2 bg-[var(--color-bg-surface-elevated)] hover:bg-[var(--color-hover-bg-strong)] border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-lg text-xs font-medium transition-colors duration-200"
                        >
                            "LinkedIn"
                        </a>
                        <a
                            href=config.github_url.as_str()
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center justify-center px-4 py-2 bg-[var(--color-bg-surface-elevated)] hover:bg-[var(--color-hover-bg-strong)] border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-lg text-xs font-medium transition-colors duration-200"
                        >
                            "GitHub"
                        </a>
                        <a
                            href=config.resume_url.as_str()
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center justify-center px-4 py-2 bg-[var(--color-bg-surface-elevated)] hover:bg-[var(--color-hover-bg-strong)] border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-lg text-xs font-medium transition-colors duration-200"
                        >
                            "Resume"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_button_labels_match_states() {
        assert_eq!(copy_button_label(CopyState::Idle), "Copy");
        assert_eq!(copy_button_label(CopyState::Copied), "Copied");
        assert_eq!(copy_button_label(CopyState::Failed), "Select to copy");
    }
}
