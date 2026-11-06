use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn About(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <section
            id="about"
            class="reveal-in card-surface p-6 md:p-8 rounded-2xl flex flex-col justify-between transition-colors duration-300"
        >
            <div>
                <h3 class="text-xl font-bold text-[var(--color-text-primary)] mb-4 border-b border-[var(--color-border-primary)] pb-3 tracking-wide">
                    "About the Developer"
                </h3>
                <p class="text-[var(--color-text-secondary)] leading-relaxed text-sm whitespace-pre-line">
                    {config.about.as_str()}
                </p>

                // Highlight Badges
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
                    {config.highlights.iter().map(|hl| {
                        view! {
                            <div class="hover-lift bg-[var(--color-bg-surface-elevated)] p-4 rounded-xl border border-[var(--color-border-primary)] hover:border-[var(--color-accent-emerald-border)] transition-all duration-300">
                                <div class="flex items-center space-x-2.5 mb-1.5">
                                    <span class="text-lg">{hl.icon.as_str()}</span>
                                    <h4 class="text-xs font-bold text-[var(--color-text-primary)] tracking-wide">
                                        {hl.label.as_str()}
                                    </h4>
                                </div>
                                <p class="text-[11px] text-[var(--color-text-tertiary)] leading-relaxed">
                                    {hl.detail.as_str()}
                                </p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
            <div class="mt-8 pt-6 border-t border-[var(--color-border-primary)]">
                <h4 class="text-xs font-bold text-[var(--color-accent-emerald)] uppercase tracking-widest mb-2">
                    "Education"
                </h4>
                <div class="flex justify-between items-center">
                    <span class="text-[var(--color-text-primary)] font-semibold">
                        {config.education.degree.as_str()}
                    </span>
                    <span class="text-[var(--color-accent-emerald)] font-semibold text-sm">
                        {config.education.year.as_str()}
                    </span>
                </div>
                <div class="text-[var(--color-text-tertiary)] text-sm mt-1">
                    {config.education.institution.as_str()}
                </div>
            </div>
        </section>
    }
}
