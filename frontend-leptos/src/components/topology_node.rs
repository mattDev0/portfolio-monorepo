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
                    "w-full p-3 rounded-lg border flex flex-col items-center justify-center text-center transition-colors duration-150 cursor-pointer select-none border-[var(--color-accent-orange)] bg-[var(--color-accent-orange-bg)]"
                }
                "java" => {
                    "w-full p-3 rounded-lg border flex flex-col items-center justify-center text-center transition-colors duration-150 cursor-pointer select-none border-[var(--color-accent)] bg-[var(--color-accent-bg)]"
                }
                "frontend" | "client" => {
                    "w-full p-3 rounded-lg border flex flex-col items-center justify-center text-center transition-colors duration-150 cursor-pointer select-none border-[var(--color-accent-gold)] bg-[var(--color-hover-bg-strong)]"
                }
                "k8s" => {
                    "w-full p-3 rounded-lg border flex flex-col items-center justify-center text-center transition-colors duration-150 cursor-pointer select-none border-[var(--color-accent-cyan)] bg-[var(--color-hover-bg-strong)]"
                }
                _ => {
                    "w-full p-3 rounded-lg border flex flex-col items-center justify-center text-center transition-colors duration-150 cursor-pointer select-none border-[var(--color-accent)] bg-[var(--color-accent-bg)]"
                }
            }
        } else {
            "w-full p-3 rounded-lg border flex flex-col items-center justify-center text-center transition-colors duration-150 cursor-pointer select-none border-[var(--color-border-primary)] bg-[var(--color-bg-surface)] hover:border-[var(--color-border-secondary)]"
        }
    };

    let title_class = move || {
        if is_active() {
            match id {
                "rust" => {
                    "text-[10px] font-semibold uppercase tracking-wider text-[var(--color-accent-orange)]"
                }
                "frontend" | "client" => {
                    "text-[10px] font-semibold uppercase tracking-wider text-[var(--color-accent-gold)]"
                }
                "k8s" => {
                    "text-[10px] font-semibold uppercase tracking-wider text-[var(--color-accent-cyan)]"
                }
                _ => {
                    "text-[10px] font-semibold uppercase tracking-wider text-[var(--color-accent)]"
                }
            }
        } else {
            "text-[10px] font-semibold uppercase tracking-wider text-[var(--color-text-secondary)]"
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
            <span class="text-base md:text-lg mb-1 text-[var(--color-text-secondary)]"><EmojiIcon glyph=icon /></span>
            <span class=title_class>{title}</span>
            <span class="text-[10px] text-[var(--color-text-muted)] font-mono mt-0.5">{tech}</span>
        </button>
    }
}
