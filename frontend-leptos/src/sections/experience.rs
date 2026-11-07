use crate::components::SectionHeading;
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn Experience(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <section
            id="experience"
            class="reveal-in card-surface p-6 md:p-8 rounded-2xl transition-colors duration-300"
        >
            <SectionHeading eyebrow="Experience" title="Professional Experience" />
            <div class="space-y-8">
                {config.experience.iter().map(|exp| {
                    view! {
                        <div class="relative pl-8 border-l-2 border-[var(--color-accent-border)] hover:border-[var(--color-accent)] transition-colors duration-300">
                            <div class="absolute w-4 h-4 bg-[var(--color-accent-solid)] rounded-full -left-[9px] top-1 border-2 border-[var(--color-bg-primary)] shadow-[0_0_8px_var(--color-accent)]"></div>
                            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-1 mb-2">
                                <h4 class="text-[var(--color-text-primary)] font-bold text-base tracking-wide">
                                    {exp.role.as_str()}
                                </h4>
                                <p class="text-[var(--color-accent)] text-xs sm:text-sm font-semibold">
                                    {exp.company.as_str()}
                                </p>
                            </div>
                            <p class="text-[var(--color-text-tertiary)] text-sm leading-relaxed">
                                {exp.description.as_str()}
                            </p>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
