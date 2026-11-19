use crate::components::EmojiIcon;
use leptos::prelude::*;

#[component]
pub fn TopologyNode(
    #[prop(into)] id: &'static str,
    #[prop(into)] icon: &'static str,
    #[prop(into)] title: &'static str,
    #[prop(into)] tech: &'static str,
    active_node: Signal<Option<&'static str>>,
    selected_node: ReadSignal<Option<&'static str>>,
    set_hovered_node: WriteSignal<Option<&'static str>>,
    set_selected_node: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    let is_active = move || active_node.get() == Some(id);

    let container_class = move || {
        if is_active() {
            match id {
                "rust" => {
                    "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-accent-orange-border)] shadow-[0_0_12px_rgba(249,115,22,0.15)] bg-[var(--color-accent-orange-bg)] scale-[1.02]"
                }
                "java" => {
                    "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-accent-rose)]/40 shadow-[0_0_12px_rgba(244,63,94,0.15)] bg-[var(--color-accent-rose)]/5 scale-[1.02]"
                }
                "frontend" | "client" => {
                    "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-accent-gold)]/40 shadow-[0_0_12px_var(--color-accent-border)] bg-[var(--color-accent-gold)]/5 scale-[1.02]"
                }
                "nginx" => {
                    "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-accent-border)] shadow-[0_0_12px_var(--color-accent-border)] bg-[var(--color-accent-bg)] scale-[1.02]"
                }
                "k8s" => {
                    "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-accent-cyan)]/50 shadow-[0_0_12px_rgba(6,182,212,0.2)] bg-[var(--color-accent-cyan)]/5 scale-[1.02]"
                }
                _ => {
                    "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-accent-border)] shadow-[0_0_12px_var(--color-accent-border)] bg-[var(--color-accent-bg)] scale-[1.02]"
                }
            }
        } else {
            "w-full p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-pointer select-none border-[var(--color-border-primary)] bg-[var(--color-bg-surface-deep)] hover:border-[var(--color-accent-border)]"
        }
    };

    let title_class = move || {
        if is_active() {
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
        <button
            type="button"
            on:mouseenter=move |_| set_hovered_node.set(Some(id))
            on:mouseleave=move |_| set_hovered_node.set(None)
            on:focus=move |_| set_hovered_node.set(Some(id))
            on:blur=move |_| set_hovered_node.set(None)
            on:click=move |ev| {
                ev.stop_propagation();
                set_selected_node.update(|cur| *cur = if *cur == Some(id) { None } else { Some(id) });
            }
            aria-pressed=move || (selected_node.get() == Some(id)).to_string()
            class=container_class
        >
            <span class="text-lg md:text-xl mb-0.5"><EmojiIcon glyph=icon /></span>
            <span class=title_class>{title}</span>
            <span class="text-[10px] text-[var(--color-text-muted)] font-mono mt-0.5">{tech}</span>
        </button>
    }
}
