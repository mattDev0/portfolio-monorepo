use crate::config::PortfolioConfig;
use leptos::prelude::*;

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

    view! {
        {move || {
            if show.get() {
                Some(view! {
                    <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-[var(--color-bg-overlay)] backdrop-blur-md transition-opacity duration-300 overflow-y-auto">
                        <div class="relative bg-[var(--color-bg-modal)] border border-[var(--color-border-secondary)] rounded-2xl w-full max-w-3xl p-6 md:p-8 shadow-2xl overflow-hidden max-h-[90vh] overflow-y-auto custom-scrollbar">
                            // Background glowing design elements
                            <div class="absolute top-0 left-1/4 w-72 h-72 bg-[var(--color-accent-bg)] rounded-full blur-3xl -z-10"></div>
                            <div class="absolute bottom-0 right-1/4 w-72 h-72 bg-[var(--color-accent-bg)] rounded-full blur-3xl -z-10"></div>

                            // Header
                            <div class="flex justify-between items-start border-b border-[var(--color-border-primary)] pb-4 mb-6">
                                <div>
                                    <span class="text-[10px] font-bold text-[var(--color-accent)] uppercase tracking-widest">
                                        "Case Study & Architecture"
                                    </span>
                                    <h3 class="text-2xl font-extrabold text-[var(--color-text-primary)] mt-1">
                                        "DevOps Control Center"
                                    </h3>
                                    <p class="text-xs text-[var(--color-text-tertiary)] mt-1">
                                        "A self-hosted telemetry, container and CI/CD dashboard."
                                    </p>
                                </div>
                                <button
                                    on:click=move |_| set_show.set(false)
                                    class="text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)] p-1 hover:bg-[var(--color-hover-bg)] rounded-lg transition-colors cursor-pointer"
                                >
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>

                            // Content Body
                            <div class="space-y-6">
                                // Overview
                                <div>
                                    <h4 class="text-xs font-bold uppercase text-[var(--color-text-tertiary)] tracking-wider mb-2">
                                        "Platform Overview"
                                    </h4>
                                    <p class="text-[var(--color-text-secondary)] text-sm leading-relaxed">
                                        "A self-hosted dashboard that unifies host and container monitoring, deployment control and CI/CD tracking into a single view, secured with JWT roles and a read-only guest mode. It runs in production on a 1 vCPU Azure VM, deployed from GitHub Actions over an SSH pipeline that validates the commit SHA before it applies anything."
                                    </p>
                                </div>

                                // Platform Evolution
                                <div>
                                    <h4 class="text-xs font-bold uppercase text-[var(--color-text-tertiary)] tracking-wider mb-2">
                                        "Platform Evolution"
                                    </h4>
                                    <p class="text-[var(--color-text-tertiary)] text-[11px] mb-3 leading-relaxed">
                                        "The platform originally ran on a self-managed K3s cluster with Traefik and cert-manager. Once the cluster's cost and operational overhead outgrew the workload, it was migrated to Docker Compose behind Caddy, with the trade-offs recorded in an ADR. Cluster-only features now stay hidden until a cluster is connected. The Kubernetes manifests remain validated in CI as a supported deployment target."
                                    </p>
                                </div>

                                // System Architecture Diagram
                                <div>
                                    <h4 class="text-xs font-bold uppercase text-[var(--color-text-tertiary)] tracking-wider mb-1">
                                        "Microservices Topology"
                                    </h4>
                                    <p class="text-[var(--color-text-tertiary)] text-[11px] mb-4 leading-relaxed">
                                        "Services run as Docker Compose containers behind " <code class="text-[var(--color-accent)] font-mono text-[10px] bg-[var(--color-accent-bg)] px-1 py-0.5 rounded">"Caddy"</code> ", which terminates TLS."
                                    </p>

                                    // Architecture Visual Grid
                                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 items-stretch justify-center my-4 py-4 px-3 border border-[var(--color-border-primary)] bg-[var(--color-hover-bg)] rounded-xl">
                                        // Client
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-bg-surface-elevated)] p-3 rounded-lg border border-[var(--color-border-primary)] text-center shadow-md">
                                            <span class="text-xl">"🌐"</span>
                                            <span class="text-[10px] font-bold text-[var(--color-text-primary)] uppercase tracking-widest mt-1">"Client Dashboard"</span>
                                            <span class="text-[10px] text-[var(--color-text-muted)] mt-1">"Vite + React UI Client"</span>
                                        </div>
                                        // Spring Gateway
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-accent-bg)] p-3 rounded-lg border border-[var(--color-accent-border)] text-center shadow-md">
                                            <span class="text-xl">"☕"</span>
                                            <span class="text-[10px] font-bold text-[var(--color-accent)] uppercase tracking-widest mt-1">"Spring Gateway"</span>
                                            <span class="text-[10px] text-[var(--color-text-tertiary)] mt-1">"JWT auth, roles & API gateway"</span>
                                        </div>
                                        // Rust Agent
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-accent-orange-bg)] p-3 rounded-lg border border-[var(--color-accent-orange-border)] text-center shadow-md">
                                            <span class="text-xl">"🦀"</span>
                                            <span class="text-[10px] font-bold text-[var(--color-accent-orange)] uppercase tracking-widest mt-1">"Rust Agent"</span>
                                            <span class="text-[10px] text-[var(--color-text-tertiary)] mt-1">"Docker & host telemetry collector"</span>
                                        </div>
                                        // Observability
                                        <div class="flex flex-col items-center justify-between bg-[var(--color-accent-bg)] p-3 rounded-lg border border-[var(--color-accent-gold)] text-center shadow-md">
                                            <span class="text-xl">"📊"</span>
                                            <span class="text-[10px] font-bold text-[var(--color-accent-gold)] uppercase tracking-widest mt-1">"Prometheus + Grafana"</span>
                                            <span class="text-[10px] text-[var(--color-text-tertiary)] mt-1">"Metrics, alert rules & dashboards"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Modal Footer Controls
                            <div class="mt-8 pt-4 border-t border-[var(--color-border-primary)] flex flex-col sm:flex-row sm:flex-wrap gap-3 justify-end items-center">
                                <a
                                    href="https://devops.mattdev0.tech"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="w-full sm:w-auto whitespace-nowrap px-5 py-2.5 bg-[var(--color-accent-solid)] hover:bg-[var(--color-accent-solid-hover)] text-white font-bold rounded-xl text-xs text-center transition-colors cursor-pointer flex items-center justify-center space-x-1.5"
                                    style="box-shadow: var(--shadow-button)"
                                >
                                    <span>"🚀"</span>
                                    <span>"Launch Live Demo (Guest Mode)"</span>
                                </a>
                                <a
                                    href="https://github.com/mattDev0/devops-control-center"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="w-full sm:w-auto whitespace-nowrap px-5 py-2.5 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] font-semibold rounded-xl text-xs text-center transition-colors cursor-pointer"
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
                                            class="w-full sm:w-auto whitespace-nowrap px-5 py-2.5 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] font-semibold rounded-xl text-xs text-center transition-colors cursor-pointer"
                                        >
                                            "Incident Write-ups"
                                        </a>
                                    })
                                } else {
                                    None
                                }}
                                <button
                                    on:click=move |_| set_show.set(false)
                                    class="w-full sm:w-auto whitespace-nowrap px-5 py-2.5 bg-[var(--color-bg-tag)] hover:bg-[var(--color-hover-bg)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] font-semibold rounded-xl text-xs cursor-pointer"
                                >
                                    "Close Case Study"
                                </button>
                            </div>

                            <p class="text-[10px] text-[var(--color-text-muted)] text-center mt-4">
                                "💡 Tip: On the live dashboard, you can bypass JWT login by clicking the \"Guest Login\" button to explore in read-only mode."
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
