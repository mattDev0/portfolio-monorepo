use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[component]
pub fn Skills(config: &'static PortfolioConfig) -> impl IntoView {
    view! {
        <section
            id="skills"
            class="reveal-in card-surface p-6 md:p-8 rounded-2xl transition-colors duration-300"
        >
            <h3 class="text-xl font-bold text-[var(--color-text-primary)] mb-6 border-b border-[var(--color-border-primary)] pb-3 tracking-wide">
                "Technical Skills"
            </h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-6">
                {config.skills.categories().into_iter().map(|(category, skills)| {
                    view! {
                        <div class="space-y-3">
                            <h4 class="text-xs font-bold text-[var(--color-accent-emerald)] uppercase tracking-wider">
                                {category}
                            </h4>
                            <div class="flex flex-wrap gap-2">
                                {skills.iter().map(|skill| {
                                    view! {
                                        <span class="bg-[var(--color-accent-emerald-bg)] border border-[var(--color-accent-emerald-border)] text-[var(--color-text-secondary)] text-xs px-2.5 py-1 rounded-lg hover:border-[var(--color-accent-emerald)] transition-colors duration-200">
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
