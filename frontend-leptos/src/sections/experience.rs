use crate::components::SectionHeading;
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn Experience(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <section
            id="experience"
            class="reveal-in card-surface p-6 md:p-8 rounded-xl"
        >
            <SectionHeading eyebrow="History" title="Experience" />
            <div class="space-y-7">
                {config.experience.iter().map(|exp| {
                    view! {
                        <div class="relative pl-6 border-l border-[var(--color-border-secondary)]">
                            <div class="absolute w-2.5 h-2.5 bg-[var(--color-accent)] rounded-full -left-[5.5px] top-1.5 ring-4 ring-[var(--color-bg-surface)]"></div>
                            <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-1 mb-1.5">
                                <h4 class="text-[var(--color-text-primary)] font-semibold text-base">
                                    {exp.role.as_str()}
                                </h4>
                                <p class="text-[var(--color-text-tertiary)] text-xs sm:text-sm font-medium">
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
