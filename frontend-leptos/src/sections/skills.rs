use crate::components::SectionHeading;
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn Skills(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <section
            id="skills"
            class="reveal-in card-surface p-6 md:p-8 rounded-xl"
        >
            <SectionHeading eyebrow="Stack" title="Technical Skills" />
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-6">
                {config.skills.categories().into_iter().map(|(category, skills)| {
                    view! {
                        <div class="space-y-3">
                            <h4 class="text-xs font-semibold text-[var(--color-text-primary)] tracking-wide">
                                {category}
                            </h4>
                            <div class="flex flex-wrap gap-1.5">
                                {skills.iter().map(|skill| {
                                    view! {
                                        <span class="bg-[var(--color-bg-tag)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] font-mono text-[11px] px-2.5 py-1 rounded-md">
                                            {skill.as_str()}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
