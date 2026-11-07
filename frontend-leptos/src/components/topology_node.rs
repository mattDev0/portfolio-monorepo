use leptos::prelude::*;

#[component]
pub fn TopologyNode(
    #[prop(into)] id: &'static str,
    #[prop(into)] icon: &'static str,
    #[prop(into)] title: &'static str,
    #[prop(into)] tech: &'static str,
    hovered_node: ReadSignal<Option<&'static str>>,
    set_hovered_node: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let is_hovered = move || hovered_node.get() == Some(id);

    let container_class = move || {
        if is_hovered() {
            match id {
                "rust" => {
                    "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-accent-orange-border)] shadow-[0_0_12px_rgba(249,115,22,0.15)] animate-pulse bg-[var(--color-accent-orange-bg)] scale-[1.02]"
                }
                "java" => {
                    "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-accent-rose)]/40 shadow-[0_0_12px_rgba(244,63,94,0.15)] animate-pulse bg-[var(--color-accent-rose)]/5 scale-[1.02]"
                }
                "frontend" | "client" => {
                    "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-accent-gold)]/40 shadow-[0_0_12px_var(--color-accent-border)] animate-pulse bg-[var(--color-accent-gold)]/5 scale-[1.02]"
                }
                "nginx" => {
                    "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-accent-border)] shadow-[0_0_12px_var(--color-accent-border)] animate-pulse bg-[var(--color-accent-bg)] scale-[1.02]"
                }
                "k8s" => {
                    "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-accent-cyan)]/50 shadow-[0_0_12px_rgba(6,182,212,0.2)] bg-[var(--color-accent-cyan)]/5 scale-[1.02]"
                }
                _ => {
                    "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-accent-border)] shadow-[0_0_12px_var(--color-accent-border)] bg-[var(--color-accent-bg)] scale-[1.02]"
                }
            }
        } else {
            "p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none border-[var(--color-border-primary)] bg-[var(--color-bg-surface-deep)] hover:border-[var(--color-accent-border)]"
        }
    };

    let title_class = move || {
        if is_hovered() {
            match id {
                "rust" => {
                    "text-[10px] font-bold uppercase tracking-wider text-[var(--color-accent-orange)]"
                }
                "java" => {
                    "text-[10px] font-bold uppercase tracking-wider text-[var(--color-accent-rose)]"
                }
                "frontend" | "client" => {
                    "text-[10px] font-bold uppercase tracking-wider text-[var(--color-accent-gold)]"
                }
                "nginx" => {
                    "text-[10px] font-bold uppercase tracking-wider text-[var(--color-accent)]"
                }
                "k8s" => {
                    "text-[10px] font-bold uppercase tracking-wider text-[var(--color-accent-cyan)]"
                }
                _ => "text-[10px] font-bold uppercase tracking-wider text-[var(--color-accent)]",
            }
        } else {
            "text-[10px] font-bold uppercase tracking-wider text-[var(--color-text-secondary)]"
        }
    };

    view! {
        <div
            on:mouseenter=move |_| set_hovered_node.set(Some(id))
            on:mouseleave=move |_| set_hovered_node.set(None)
            class=container_class
        >
            <span class="text-lg md:text-xl mb-0.5">{icon}</span>
            <span class=title_class>{title}</span>
            <span class="text-[10px] text-[var(--color-text-muted)] font-mono mt-0.5">{tech}</span>
        </div>
    }
}
