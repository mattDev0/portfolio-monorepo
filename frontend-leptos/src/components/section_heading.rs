use leptos::prelude::*;

#[component]
pub fn SectionHeading(eyebrow: &'static str, title: &'static str) -> impl IntoView {
    view! {
        <div class="mb-6 pb-4 border-b border-[var(--color-border-primary)]">
            <p class="font-mono text-[11px] font-semibold uppercase tracking-[0.2em] text-[var(--color-accent-gold)] mb-1.5">
                {eyebrow}
            </p>
            <h3 class="text-2xl md:text-3xl font-extrabold tracking-tight text-[var(--color-text-primary)]">
                {title}
            </h3>
        </div>
    }
}
