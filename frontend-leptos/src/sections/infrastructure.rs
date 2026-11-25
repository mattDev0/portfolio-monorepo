use crate::components::{
    DotAccent, FeedDot, FeedPending, GitHubActivity, Icon, SectionHeading, Sparkline,
    SpotifyPlayer, StaleNotice, TopologyNode,
};
use crate::constants::get_topology_info;
use crate::spotify::SpotifySignals;
use crate::telemetry::TelemetrySignals;
use leptos::prelude::*;

#[component]
pub fn Infrastructure(telemetry: TelemetrySignals, spotify: SpotifySignals) -> impl IntoView {
    let (hovered_node, set_hovered_node) = signal(None::<&'static str>);
    let (selected_node, set_selected_node) = signal(None::<&'static str>);
    let active_topology_node =
        Signal::derive(move || hovered_node.get().or_else(|| selected_node.get()));

    let rust_status = telemetry.rust_status;
    let java_status = telemetry.java_status;
    let telemetry_history = telemetry.telemetry_history;
    let network_status = telemetry.network_status;
    let network_history = telemetry.network_history;
    let rust_health = telemetry.rust_health;
    let java_health = telemetry.java_health;
    let network_health = telemetry.network_health;

    let k8s_box_class = move || {
        if active_topology_node.get() == Some("k8s") {
            "flex-[2] w-full p-4 rounded-lg border border-dashed transition-colors duration-200 border-[var(--color-accent)] bg-[var(--color-accent-bg)] cursor-pointer"
        } else {
            "flex-[2] w-full p-4 rounded-lg border border-dashed transition-colors duration-200 border-[var(--color-border-secondary)] bg-[var(--color-bg-surface-deep)] cursor-pointer"
        }
    };

    view! {
        <section id="infrastructure" class="reveal-in space-y-6">
            <SectionHeading
                eyebrow="Observability"
                title="Live Infrastructure"
                subtitle="Real-time health telemetry and network architecture mapping my deployed systems."
            />

            // Architecture Topology - Full Width
            <div class="card-surface p-6 md:p-7 rounded-xl">
                <div class="border-b border-[var(--color-border-primary)] pb-3 mb-6">
                    <span class="text-[11px] font-medium text-[var(--color-text-muted)] uppercase tracking-[0.12em] font-mono">
                        "Deployment Topology"
                    </span>
                    <h4 class="text-base sm:text-lg font-bold text-[var(--color-text-primary)] tracking-tight mt-0.5">
                        "Microservices Architecture"
                    </h4>
                </div>

                <div class="flex flex-col lg:flex-row gap-6 items-center lg:items-stretch">
                    // Left/Main column: The diagram
                    <div class="flex-grow w-full flex flex-col gap-5 justify-center">
                        <div class="flex flex-col md:flex-row items-center md:items-stretch gap-3.5 justify-between">
                            // 1. Client Card
                            <div class="flex-1 w-full flex flex-col justify-center">
                                <TopologyNode
                                    id="client"
                                    icon="🌐"
                                    title="Client Browser"
                                    tech="Rust + Leptos"
                                    active_node=active_topology_node
                                    selected_node=selected_node
                                    set_hovered_node=set_hovered_node
                                    set_selected_node=set_selected_node
                                />
                            </div>

                            // Arrow Client -> Nginx
                            <div class="flex items-center justify-center text-[var(--color-text-muted)] font-mono text-xs py-1 md:py-0">
                                <span class="md:hidden"><Icon name="arrow-down" /></span>
                                <span class="hidden md:inline text-[var(--color-text-muted)]">
                                    "── HTTPS ──▶"
                                </span>
                            </div>

                            // 2. Nginx Card
                            <div class="flex-1 w-full flex flex-col justify-center">
                                <TopologyNode
                                    id="nginx"
                                    icon="🛡️"
                                    title="GCP Ingress"
                                    tech="HTTPS Gateway"
                                    active_node=active_topology_node
                                    selected_node=selected_node
                                    set_hovered_node=set_hovered_node
                                    set_selected_node=set_selected_node
                                />
                            </div>

                            // Arrow Nginx -> K8s Namespace
                            <div class="flex items-center justify-center text-[var(--color-text-muted)] font-mono text-xs py-1 md:py-0">
                                <span class="md:hidden"><Icon name="arrow-down" /></span>
                                <span class="hidden md:inline text-[var(--color-text-muted)]">
                                    "── Proxy ──▶"
                                </span>
                            </div>

                            // 3. Google Cloud Run (Serverless)
                            <div
                                on:mouseenter=move |_| set_hovered_node.set(Some("k8s"))
                                on:mouseleave=move |_| set_hovered_node.set(None)
                                on:click=move |_| {
                                    set_selected_node.update(|cur| *cur = if *cur == Some("k8s") { None } else { Some("k8s") });
                                }
                                class=k8s_box_class
                            >
                                <div class="flex justify-between items-center mb-3 gap-2">
                                    <span class="text-[10px] font-medium text-[var(--color-text-muted)] uppercase tracking-widest font-mono">
                                        "Platform: Google Cloud"
                                    </span>
                                    <button
                                        type="button"
                                        on:mouseenter=move |_| set_hovered_node.set(Some("k8s"))
                                        on:mouseleave=move |_| set_hovered_node.set(None)
                                        on:focus=move |_| set_hovered_node.set(Some("k8s"))
                                        on:blur=move |_| set_hovered_node.set(None)
                                        on:click=move |ev| {
                                            ev.stop_propagation();
                                            set_selected_node.update(|cur| *cur = if *cur == Some("k8s") { None } else { Some("k8s") });
                                        }
                                        aria-pressed=move || (selected_node.get() == Some("k8s")).to_string()
                                        class="text-[10px] font-mono text-[var(--color-text-secondary)] bg-[var(--color-bg-tag)] hover:bg-[var(--color-hover-bg-strong)] border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] px-1.5 py-0.5 rounded cursor-pointer inline-flex items-center transition-colors"
                                    >
                                        <Icon name="cloud" class="w-3 h-3 mr-1" />"GCP Cloud Run"
                                    </button>
                                </div>

                                <div class="grid grid-cols-1 sm:grid-cols-3 gap-2">
                                    <TopologyNode
                                        id="frontend"
                                        icon="🦀"
                                        title="Frontend Service"
                                        tech="Cloud Run"
                                        active_node=active_topology_node
                                        selected_node=selected_node
                                        set_hovered_node=set_hovered_node
                                        set_selected_node=set_selected_node
                                    />
                                    <TopologyNode
                                        id="rust"
                                        icon="🦀"
                                        title="Rust Service"
                                        tech="Cloud Run"
                                        active_node=active_topology_node
                                        selected_node=selected_node
                                        set_hovered_node=set_hovered_node
                                        set_selected_node=set_selected_node
                                    />
                                    <TopologyNode
                                        id="java"
                                        icon="☕"
                                        title="Java Service"
                                        tech="Cloud Run"
                                        active_node=active_topology_node
                                        selected_node=selected_node
                                        set_hovered_node=set_hovered_node
                                        set_selected_node=set_selected_node
                                    />
                                </div>
                            </div>
                        </div>

                        // Connections from Cloud Run to external services
                        <div class="flex flex-col md:flex-row gap-4 justify-between items-stretch">
                            // Arrow Rust -> Spotify
                            <div class="flex-1 bg-[var(--color-bg-surface-deep)] border border-[var(--color-border-primary)] rounded-lg p-3.5 flex flex-col justify-between">
                                <div class="flex justify-between items-center text-[10px] text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
                                    <span>"Rust Gateway"</span>
                                    <span class="text-[var(--color-accent-orange)] font-mono">"Axum Outbound"</span>
                                </div>
                                <div class="flex items-center justify-between gap-2">
                                    <span class="text-xs font-semibold text-[var(--color-text-secondary)]">"Rust API"</span>
                                    <span class="text-[var(--color-text-muted)] font-mono text-[10px] shrink-0">"── OAuth ──▶"</span>
                                    <div class="flex-1 max-w-[160px]">
                                        <TopologyNode
                                            id="spotify"
                                            icon="🎵"
                                            title="Spotify API"
                                            tech="v1 Player Web Service"
                                            active_node=active_topology_node
                                            selected_node=selected_node
                                            set_hovered_node=set_hovered_node
                                            set_selected_node=set_selected_node
                                        />
                                    </div>
                                </div>
                            </div>

                            // Arrow Java -> GitHub
                            <div class="flex-1 bg-[var(--color-bg-surface-deep)] border border-[var(--color-border-primary)] rounded-lg p-3.5 flex flex-col justify-between">
                                <div class="flex justify-between items-center text-[10px] text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
                                    <span>"Java Caching"</span>
                                    <span class="text-[var(--color-accent)] font-mono">"Spring Cacheable"</span>
                                </div>
                                <div class="flex items-center justify-between gap-2">
                                    <span class="text-xs font-semibold text-[var(--color-text-secondary)]">"Java API"</span>
                                    <span class="text-[var(--color-text-muted)] font-mono text-[10px] shrink-0">"── REST ──▶"</span>
                                    <div class="flex-1 max-w-[160px]">
                                        <TopologyNode
                                            id="github"
                                            icon="🐙"
                                            title="GitHub API"
                                            tech="v3 Public REST"
                                            active_node=active_topology_node
                                            selected_node=selected_node
                                            set_hovered_node=set_hovered_node
                                            set_selected_node=set_selected_node
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Right/Inspector column: The Topology Inspector
                    <div class="w-full lg:w-80 flex-shrink-0 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] rounded-lg p-5 flex flex-col justify-between">
                        <div>
                            <div class="flex items-center space-x-2 border-b border-[var(--color-border-primary)] pb-2 mb-3">
                                <span class="inline-flex rounded-full h-2 w-2 bg-[var(--color-status-online)]"></span>
                                <h4 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wider">
                                    "Topology Inspector"
                                </h4>
                            </div>

                            {move || {
                                if let Some(node_id) = active_topology_node.get() {
                                    if let Some(info) = get_topology_info(node_id) {
                                        return view! {
                                            <div class="space-y-3.5">
                                                <div class="flex justify-between items-center gap-2">
                                                    <span class="text-sm font-bold text-[var(--color-text-primary)]">
                                                        {info.title}
                                                    </span>
                                                    <span class="bg-[var(--color-bg-tag)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] text-[10px] px-1.5 py-0.5 rounded font-mono uppercase tracking-wider shrink-0">
                                                        {info.badge}
                                                    </span>
                                                </div>

                                                <div class="font-mono text-[10px] space-y-1 bg-[var(--color-bg-surface-inset)] p-2.5 rounded-md border border-[var(--color-border-primary)]">
                                                    <div class="flex justify-between gap-2">
                                                        <span class="text-[var(--color-text-muted)]">"ENGINE:"</span>
                                                        <span class="text-[var(--color-text-secondary)] text-right">{info.tech}</span>
                                                    </div>
                                                    <div class="flex justify-between gap-2 mt-1">
                                                        <span class="text-[var(--color-text-muted)]">"PORT/PROTO:"</span>
                                                        <span class="text-[var(--color-accent)] font-semibold text-right">{info.protocol}</span>
                                                    </div>
                                                </div>

                                                <p class="text-[var(--color-text-tertiary)] text-xs leading-relaxed">
                                                    {info.description}
                                                </p>
                                            </div>
                                        }.into_any();
                                    }
                                }
                                view! {
                                    <div class="text-center py-10 flex flex-col items-center justify-center space-y-2.5">
                                        <span class="text-xl text-[var(--color-text-muted)]"><Icon name="search" /></span>
                                        <p class="text-[var(--color-text-muted)] text-xs leading-relaxed font-mono">
                                            "Hover or tap any node in the diagram to inspect microservice details, routes, and deployment states."
                                        </p>
                                    </div>
                                }.into_any()
                            }}
                        </div>

                        <div class="mt-6 pt-3 border-t border-[var(--color-border-primary)] text-[10px] text-[var(--color-text-muted)] font-mono flex flex-wrap items-center justify-between gap-x-4 gap-y-1">
                            <span class="whitespace-nowrap">"DEPLOYMENT: live"</span>
                            <span>"PLATFORM: Cloud Run (us-central1)"</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // Left/Spans 2 columns: GitHub Commits
                <div class="lg:col-span-2 flex flex-col gap-6">
                    <GitHubActivity />
                </div>

                // Right column: Live system telemetry & Spotify
                <div class="lg:col-span-1 flex flex-col gap-5 font-mono">
                    // Rust Engine
                    <section class="card-surface p-5 rounded-xl relative">
                        <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                            <FeedDot health=rust_health accent=DotAccent::Orange />
                            <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-transform duration-150 origin-top-right rounded-md bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-secondary)] p-2 text-center text-[10px] text-[var(--color-text-primary)] font-mono shadow-md z-20">
                                "Live Axum Service"
                            </div>
                        </div>
                        <h3 class="text-base font-bold text-[var(--color-text-primary)] mb-0.5 tracking-tight font-sans">
                            "Rust Engine"
                        </h3>
                        <p class="text-[11px] text-[var(--color-text-muted)] font-sans mb-4">
                            "Low-level OS telemetry & Spotify API gateway"
                        </p>
                        <StaleNotice health=rust_health />

                        {move || {
                            if let Some(rust) = rust_status.get() {
                                let cpu_points = telemetry_history.get().into_iter().map(|h| h.cpu).collect::<Vec<_>>();
                                let mem_points = telemetry_history.get().into_iter().map(|h| h.memory).collect::<Vec<_>>();
                                let cpu_str = format!("{:.1}%", rust.cpu_usage_percent);
                                let cpu_width = format!("width: {}%", rust.cpu_usage_percent);

                                view! {
                                    <div class="font-mono text-xs space-y-3.5 mt-2">
                                        <div class="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"OS"</span>
                                            <span class="text-[var(--color-text-primary)] font-medium">{rust.os_info}</span>
                                        </div>

                                        <div class="border-b border-[var(--color-border-primary)] pb-2">
                                            <div class="flex justify-between items-center mb-1.5">
                                                <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"CPU Utilization"</span>
                                                <div class="flex items-center space-x-3">
                                                    <Sparkline data=Signal::derive(move || cpu_points.clone()) color="var(--color-accent-orange)" max=100.0 />
                                                    <span class="text-[var(--color-accent-orange)] font-semibold">
                                                        {cpu_str}
                                                    </span>
                                                </div>
                                            </div>
                                            <div class="w-full h-1.5 bg-[var(--color-bg-progress-track)] rounded-full overflow-hidden">
                                                <div
                                                    class="h-full bg-[var(--color-accent-orange)] transition-all duration-700 ease-out"
                                                    style=cpu_width
                                                ></div>
                                            </div>
                                        </div>

                                        <div class="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Threads"</span>
                                            <span class="text-[var(--color-text-primary)] font-medium">
                                                {format!("{} Logical Cores", rust.cpu_core_count)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between items-center">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Memory"</span>
                                            <div class="flex items-center space-x-3">
                                                <Sparkline data=Signal::derive(move || mem_points.clone()) color="var(--color-accent-amber)" max=100.0 />
                                                <span class="text-[var(--color-text-primary)] font-medium">
                                                    {format!("{} MB / {} MB", rust.memory_used_mb, rust.memory_total_mb)}
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <FeedPending health=rust_health label="Handshake pending..." /> }.into_any()
                            }
                        }}
                    </section>

                    // Java Infrastructure
                    <section class="card-surface p-5 rounded-xl relative">
                        <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                            <FeedDot health=java_health accent=DotAccent::Online />
                            <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-transform duration-150 origin-top-right rounded-md bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-secondary)] p-2 text-center text-[10px] text-[var(--color-text-primary)] font-mono shadow-md z-20">
                                "Live Spring Service"
                            </div>
                        </div>
                        <h3 class="text-base font-bold text-[var(--color-text-primary)] mb-0.5 tracking-tight font-sans">
                            "Java Infrastructure"
                        </h3>
                        <p class="text-[11px] text-[var(--color-text-muted)] font-sans mb-4">
                            "Spring Cache engine driving GitHub API events"
                        </p>
                        <StaleNotice health=java_health />

                        {move || {
                            if let Some(java) = java_status.get() {
                                view! {
                                    <div class="font-mono text-xs space-y-3.5 mt-2">
                                        <div class="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Version"</span>
                                            <span class="text-[var(--color-text-primary)] font-medium">{java.engine}</span>
                                        </div>
                                        <div class="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Uptime"</span>
                                            <span class="text-[var(--color-accent)] font-semibold">
                                                {format!("{}h {}m", java.uptime_hours, java.uptime_minutes)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Active Threads"</span>
                                            <span class="text-[var(--color-text-primary)] font-medium">
                                                {format!("{} Threads", java.active_threads)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"JVM Memory"</span>
                                            <span class="text-[var(--color-text-primary)] font-medium">
                                                {format!("{} MB / {} MB", java.jvm_memory_used_mb, java.jvm_memory_total_mb)}
                                            </span>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <FeedPending health=java_health label="Handshake pending..." /> }.into_any()
                            }
                        }}
                    </section>

                    // Network Telemetry
                    <section class="card-surface p-5 rounded-xl relative">
                        <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                            <FeedDot health=network_health accent=DotAccent::Online />
                            <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-transform duration-150 origin-top-right rounded-md bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-secondary)] p-2 text-center text-[10px] text-[var(--color-text-primary)] font-mono shadow-md z-20">
                                "Blackbox Exporter"
                            </div>
                        </div>
                        <h3 class="text-base font-bold text-[var(--color-text-primary)] mb-0.5 tracking-tight font-sans">
                            "Network Telemetry"
                        </h3>
                        <p class="text-[11px] text-[var(--color-text-muted)] font-sans mb-4">
                            "Synthetic latency probes via ICMP (Ping)"
                        </p>
                        <StaleNotice health=network_health />

                        {move || {
                            if let Some(net) = network_status.get() {
                                let google_hist = network_history.get().into_iter().map(|h| h.google_dns).collect::<Vec<_>>();
                                let cloudflare_hist = network_history.get().into_iter().map(|h| h.cloudflare_dns).collect::<Vec<_>>();
                                let riot_hist = network_history.get().into_iter().map(|h| h.riot_games).collect::<Vec<_>>();

                                let google_dot_class = if net.google_dns.status == "online" {
                                    "h-2 w-2 rounded-full bg-[var(--color-status-online)]"
                                } else {
                                    "h-2 w-2 rounded-full bg-[var(--color-status-error)]"
                                };

                                let cf_dot_class = if net.cloudflare_dns.status == "online" {
                                    "h-2 w-2 rounded-full bg-[var(--color-status-online)]"
                                } else {
                                    "h-2 w-2 rounded-full bg-[var(--color-status-error)]"
                                };

                                let riot_dot_class = if net.riot_games.status == "online" {
                                    "h-2 w-2 rounded-full bg-[var(--color-status-online)]"
                                } else {
                                    "h-2 w-2 rounded-full bg-[var(--color-status-error)]"
                                };

                                view! {
                                    <div class="font-mono text-xs space-y-3.5 mt-2">
                                        // Google DNS
                                        <div class="flex justify-between items-center border-b border-[var(--color-border-primary)] pb-2">
                                            <div class="flex items-center space-x-2">
                                                <span class=google_dot_class></span>
                                                <span class="text-[var(--color-text-primary)]">{net.google_dns.name}</span>
                                                <span class="text-[10px] text-[var(--color-text-muted)]">{format!("({})", net.google_dns.target)}</span>
                                            </div>
                                            <div class="flex items-center space-x-3">
                                                <Sparkline data=Signal::derive(move || google_hist.clone()) color="var(--color-accent)" />
                                                <span class="text-[var(--color-text-primary)] font-semibold w-14 text-right">
                                                    {format!("{} ms", net.google_dns.latency_ms)}
                                                </span>
                                            </div>
                                        </div>

                                        // Cloudflare DNS
                                        <div class="flex justify-between items-center border-b border-[var(--color-border-primary)] pb-2">
                                            <div class="flex items-center space-x-2">
                                                <span class=cf_dot_class></span>
                                                <span class="text-[var(--color-text-primary)]">{net.cloudflare_dns.name}</span>
                                                <span class="text-[10px] text-[var(--color-text-muted)]">{format!("({})", net.cloudflare_dns.target)}</span>
                                            </div>
                                            <div class="flex items-center space-x-3">
                                                <Sparkline data=Signal::derive(move || cloudflare_hist.clone()) color="var(--color-accent)" />
                                                <span class="text-[var(--color-text-primary)] font-semibold w-14 text-right">
                                                    {format!("{} ms", net.cloudflare_dns.latency_ms)}
                                                </span>
                                            </div>
                                        </div>

                                        // Riot Games NA
                                        <div class="flex justify-between items-center">
                                            <div class="flex items-center space-x-2">
                                                <span class=riot_dot_class></span>
                                                <span class="text-[var(--color-text-primary)]">{net.riot_games.name}</span>
                                                <span class="text-[10px] text-[var(--color-text-muted)]">{format!("({})", net.riot_games.target)}</span>
                                            </div>
                                            <div class="flex items-center space-x-3">
                                                <Sparkline data=Signal::derive(move || riot_hist.clone()) color="var(--color-accent)" />
                                                <span class="text-[var(--color-text-primary)] font-semibold w-14 text-right">
                                                    {format!("{} ms", net.riot_games.latency_ms)}
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <FeedPending health=network_health label="Ping diagnostics pending..." /> }.into_any()
                            }
                        }}
                    </section>

                    // Live Spotify Session via Rust
                    <SpotifyPlayer
                        spotify_data=spotify.spotify_data
                        local_progress_ms=spotify.local_progress_ms
                        spotify_health=spotify.spotify_health
                    />
                </div>
            </div>
        </section>
    }
}
