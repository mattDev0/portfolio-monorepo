use leptos::prelude::*;

#[component]
pub fn SectionHeading(
    eyebrow: &'static str,
    title: &'static str,
    #[prop(optional)] subtitle: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="mb-6 pb-3.5 border-b border-[var(--color-border-primary)]">
            <p class="font-mono text-[11px] font-medium uppercase tracking-[0.14em] text-[var(--color-text-muted)] mb-1">
                {eyebrow}
            </p>
            <h3 class="text-xl md:text-2xl font-bold tracking-tight text-[var(--color-text-primary)]">
                {title}
            </h3>
            {subtitle.map(|sub| view! {
                <p class="text-xs sm:text-sm text-[var(--color-text-tertiary)] mt-1">{sub}</p>
            })}
        </div>
    }
}
