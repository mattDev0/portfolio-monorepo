use crate::components::{EmojiIcon, SectionHeading};
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn About(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <section
            id="about"
            class="reveal-in card-surface p-6 md:p-8 rounded-xl flex flex-col justify-between"
        >
            <div>
                <SectionHeading eyebrow="Overview" title="About" />
                <p class="text-[var(--color-text-secondary)] leading-relaxed text-sm sm:text-[15px] whitespace-pre-line">
                    {config.about.as_str()}
                </p>

                // Highlight Cards
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
                    {config.highlights.iter().map(|hl| {
                        view! {
                            <div class="bg-[var(--color-bg-surface-elevated)] p-4 rounded-lg border border-[var(--color-border-primary)]">
                                <div class="flex items-center space-x-2 mb-1.5">
                                    <span class="text-base text-[var(--color-accent)]"><EmojiIcon glyph=hl.icon.as_str() /></span>
                                    <h4 class="text-xs font-semibold text-[var(--color-text-primary)] tracking-wide">
                                        {hl.label.as_str()}
                                    </h4>
                                </div>
                                <p class="text-xs text-[var(--color-text-tertiary)] leading-relaxed">
                                    {hl.detail.as_str()}
                                </p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
            <div class="mt-7 pt-5 border-t border-[var(--color-border-primary)]">
                <h4 class="font-mono text-[11px] font-medium text-[var(--color-text-muted)] uppercase tracking-[0.14em] mb-2">
                    "Education"
                </h4>
                <div class="flex justify-between items-baseline gap-4">
                    <span class="text-[var(--color-text-primary)] font-semibold text-sm sm:text-base">
                        {config.education.degree.as_str()}
                    </span>
                    <span class="font-mono text-[var(--color-text-tertiary)] text-xs shrink-0">
                        {config.education.year.as_str()}
                    </span>
                </div>
                <div class="text-[var(--color-text-tertiary)] text-sm mt-0.5">
                    {config.education.institution.as_str()}
                </div>
            </div>
        </section>
    }
}
