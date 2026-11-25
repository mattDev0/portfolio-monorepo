use crate::components::{EmojiIcon, Icon};
use crate::config::PortfolioConfig;
use leptos::prelude::*;

#[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
#[component]
pub fn CaseStudyModal(
    show: Signal<bool>,
    set_show: WriteSignal<bool>,
    config: &'static PortfolioConfig,
) -> impl IntoView {
    let devops_project = config
        .projects
        .iter()
        .find(|p| p.title == "DevOps Control Center");

    #[cfg(target_arch = "wasm32")]
    {
        let key_handle = window_event_listener(leptos::ev::keydown, move |ev| {
            if show.get_untracked() && ev.key() == "Escape" {
                set_show.set(false);
            }
        });
        on_cleanup(move || key_handle.remove());

        Effect::new(move |_| {
            let is_open = show.get();
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                let style = body.style();
                if is_open {
                    let _ = style.set_property("overflow", "hidden");
                } else {
                    let _ = style.remove_property("overflow");
                }
            }
        });

        on_cleanup(move || {
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                let _ = body.style().remove_property("overflow");
            }
        });
    }

    view! {
        {move || {
            if show.get() {
                Some(view! {
                    <div
                        on:click=move |_| set_show.set(false)
                        class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-[var(--color-bg-overlay)] backdrop-blur-sm transition-opacity duration-200 overflow-y-auto"
                    >
                        <div
                            role="dialog"
                            aria-modal="true"
                            aria-labelledby="case-study-modal-title"
                            on:click=move |ev| ev.stop_propagation()
                            class="relative bg-[var(--color-bg-modal)] border border-[var(--color-border-secondary)] rounded-xl w-full max-w-3xl p-6 md:p-8 max-h-[90vh] overflow-y-auto custom-scrollbar"
                            style="box-shadow: var(--shadow-elevated)"
                        >
                            // Header
                            <div class="flex justify-between items-start border-b border-[var(--color-border-primary)] pb-4 mb-6">
                                <div>
                                    <span class="font-mono text-[11px] font-medium text-[var(--color-text-muted)] uppercase tracking-[0.12em]">
                                        "Case Study & Architecture"
                                    </span>
                                    <h3
                                        id="case-study-modal-title"
                                        class="text-2xl font-bold text-[var(--color-text-primary)] tracking-tight mt-1"
                                    >
                                        "DevOps Control Center"
                                    </h3>
                                    <p class="text-xs text-[var(--color-text-tertiary)] mt-1">
                                        "A self-hosted telemetry, container and CI/CD dashboard."
                                    </p>
                                </div>
                                <button
                                    type="button"
                                    aria-label="Close case study"
                                    on:click=move |_| set_show.set(false)
                                    class="text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)] p-1.5 hover:bg-[var(--color-hover-bg-strong)] rounded-lg transition-colors cursor-pointer"
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>

                            // Content Body
                            <div class="space-y-6">
                                // Overview
                                <div>
                                    <h4 class="font-mono text-[11px] font-medium uppercase text-[var(--color-text-muted)] tracking-[0.12em] mb-2">
                                        "Platform Overview"
                                    </h4>
                                    <p class="text-[var(--color-text-secondary)] text-sm leading-relaxed">
                                        "A self-hosted dashboard that unifies host and container monitoring, deployment control and CI/CD tracking into a single view, secured with JWT roles and a read-only guest mode. It runs in production on a 1 vCPU Azure VM, deployed from GitHub Actions over an SSH pipeline that validates the commit SHA before it applies anything."
                                    </p>
                                </div>

                                // Platform Evolution
                                <div>
                                    <h4 class="font-mono text-[11px] font-medium uppercase text-[var(--color-text-muted)] tracking-[0.12em] mb-2">
                                        "Platform Evolution"
                                    </h4>
                                    <p class="text-[var(--color-text-tertiary)] text-xs sm:text-sm leading-relaxed">
                                        "The platform originally ran on a self-managed K3s cluster with Traefik and cert-manager. Once the cluster's cost and operational overhead outgrew the workload, it was migrated to Docker Compose behind Caddy, with the trade-offs recorded in an ADR. Cluster-only features now stay hidden until a cluster is connected. The Kubernetes manifests remain validated in CI as a supported deployment target."
                                    </p>
                                </div>

                                // System Architecture Diagram
                                <div>
                                    <h4 class="font-mono text-[11px] font-medium uppercase text-[var(--color-text-muted)] tracking-[0.12em] mb-1">
                                        "Microservices Topology"
                                    </h4>
                                    <p class="text-[var(--color-text-tertiary)] text-xs mb-4 leading-relaxed">
                                        "Services run as Docker Compose containers behind " <code class="text-[var(--color-accent)] font-mono text-[11px] bg-[var(--color-bg-tag)] px-1.5 py-0.5 rounded">"Caddy"</code> ", which terminates TLS."
                                    </p>

                                    // Architecture Visual Grid
                                    <div class="grid grid-cols-2 md:grid-cols-4 gap-3 items-stretch my-4 p-3.5 border border-[var(--color-border-primary)] bg-[var(--color-bg-surface-deep)] rounded-lg">
                                        // Client
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-bg-surface)] p-3 rounded-md border border-[var(--color-border-primary)] text-center">
                                            <span class="text-lg text-[var(--color-text-secondary)]"><EmojiIcon glyph="🌐" /></span>
                                            <span class="text-[11px] font-semibold text-[var(--color-text-primary)] mt-1.5">"Client Dashboard"</span>
                                            <span class="text-[10px] text-[var(--color-text-muted)] mt-1">"Vite + React UI Client"</span>
                                        </div>
                                        // Spring Gateway
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-bg-surface)] p-3 rounded-md border border-[var(--color-border-primary)] text-center">
                                            <span class="text-lg text-[var(--color-accent)]"><EmojiIcon glyph="☕" /></span>
                                            <span class="text-[11px] font-semibold text-[var(--color-text-primary)] mt-1.5">"Spring Gateway"</span>
                                            <span class="text-[10px] text-[var(--color-text-muted)] mt-1">"JWT auth, roles & API gateway"</span>
                                        </div>
                                        // Rust Agent
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-bg-surface)] p-3 rounded-md border border-[var(--color-border-primary)] text-center">
                                            <span class="text-lg text-[var(--color-accent-orange)]"><EmojiIcon glyph="🦀" /></span>
                                            <span class="text-[11px] font-semibold text-[var(--color-text-primary)] mt-1.5">"Rust Agent"</span>
                                            <span class="text-[10px] text-[var(--color-text-muted)] mt-1">"Docker & host telemetry collector"</span>
                                        </div>
                                        // Observability
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-bg-surface)] p-3 rounded-md border border-[var(--color-border-primary)] text-center">
                                            <span class="text-lg text-[var(--color-accent-gold)]"><EmojiIcon glyph="📊" /></span>
                                            <span class="text-[11px] font-semibold text-[var(--color-text-primary)] mt-1.5">"Prometheus + Grafana"</span>
                                            <span class="text-[10px] text-[var(--color-text-muted)] mt-1">"Metrics, alert rules & dashboards"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Modal Footer Controls
                            <div class="mt-7 pt-4 border-t border-[var(--color-border-primary)] flex flex-col sm:flex-row sm:flex-wrap gap-2.5 justify-end items-center">
                                <a
                                    href="https://devops.mattdev0.tech"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="w-full sm:w-auto whitespace-nowrap px-4 py-2 bg-[var(--color-accent-solid)] hover:bg-[var(--color-accent-solid-hover)] text-white font-semibold rounded-lg text-xs text-center transition-colors cursor-pointer flex items-center justify-center space-x-1.5"
                                >
                                    <Icon name="play" />
                                    <span>"Launch Live Demo (Guest Mode)"</span>
                                </a>
                                <a
                                    href="https://github.com/mattDev0/devops-control-center"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="w-full sm:w-auto whitespace-nowrap px-4 py-2 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] font-medium rounded-lg text-xs text-center transition-colors cursor-pointer"
                                >
                                    "View Repository Code"
                                </a>
                                {if let Some(incidents) = devops_project.and_then(|p| p.incidents_link.as_ref()) {
                                    let incidents_url = incidents.clone();
                                    Some(view! {
                                        <a
                                            href=incidents_url
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="w-full sm:w-auto whitespace-nowrap px-4 py-2 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg-strong)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] font-medium rounded-lg text-xs text-center transition-colors cursor-pointer"
                                        >
                                            "Incident Write-ups"
                                        </a>
                                    })
                                } else {
                                    None
                                }}
                                <button
                                    type="button"
                                    on:click=move |_| set_show.set(false)
                                    class="w-full sm:w-auto whitespace-nowrap px-4 py-2 bg-[var(--color-bg-tag)] hover:bg-[var(--color-hover-bg-strong)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] font-medium rounded-lg text-xs cursor-pointer transition-colors"
                                >
                                    "Close"
                                </button>
                            </div>

                            <p class="text-[11px] text-[var(--color-text-muted)] text-center mt-4">
                                <Icon name="lightbulb" class="w-3 h-3 mr-1" />"Tip: On the live dashboard, you can bypass JWT login by clicking the \"Guest Login\" button to explore in read-only mode."
                            </p>
                        </div>
                    </div>
                })
            } else {
                None
            }
        }}
    }
}
