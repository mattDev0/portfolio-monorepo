use crate::components::{
    DotAccent, FeedDot, FeedPending, GitHubActivity, Icon, Sparkline, SpotifyPlayer, StaleNotice,
    TopologyNode,
};
use crate::constants::get_topology_info;
use crate::spotify::SpotifySignals;
use crate::telemetry::TelemetrySignals;
use leptos::prelude::*;

#[component]
pub fn Infrastructure(telemetry: TelemetrySignals, spotify: SpotifySignals) -> impl IntoView {
    let (hovered_topology_node, set_hovered_topology_node) = signal(None::<&'static str>);

    let rust_status = telemetry.rust_status;
    let java_status = telemetry.java_status;
    let telemetry_history = telemetry.telemetry_history;
    let network_status = telemetry.network_status;
    let network_history = telemetry.network_history;
    let rust_health = telemetry.rust_health;
    let java_health = telemetry.java_health;
    let network_health = telemetry.network_health;

    let k8s_box_class = move || {
        if hovered_topology_node.get() == Some("k8s") {
            "flex-[2] w-full p-4 rounded-xl border border-dashed transition-all duration-300 border-[var(--color-accent-gold)] bg-[var(--color-accent-bg)] shadow-[0_0_15px_var(--color-accent-border)]"
        } else {
            "flex-[2] w-full p-4 rounded-xl border border-dashed transition-all duration-300 border-[var(--color-border-secondary)] bg-[var(--color-bg-surface-deep)]"
        }
    };

    view! {
        <section id="infrastructure" class="reveal-in space-y-8">
            <div class="border-b border-[var(--color-border-primary)] pb-3">
                <span class="text-[10px] font-bold text-[var(--color-accent)] uppercase tracking-widest font-mono">
                    "Systems Observability"
                </span>
                <h3 class="text-2xl font-extrabold text-[var(--color-text-primary)] tracking-wide mt-1">
                    "Live Infrastructure Dashboard"
                </h3>
                <p class="text-xs text-[var(--color-text-tertiary)] mt-1">
                    "Real-time health telemetry and network architecture mapping my deployed systems."
                </p>
            </div>

            // Architecture Topology - Full Width
            <div class="bg-[var(--color-bg-surface-deep)] backdrop-blur-md p-6 md:p-8 rounded-2xl border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] transition-colors duration-300">
                <div class="border-b border-[var(--color-border-primary)] pb-3 mb-6">
                    <span class="text-[10px] font-bold text-[var(--color-accent)] uppercase tracking-widest font-mono">
                        "Deployment Topology"
                    </span>
                    <h4 class="text-lg font-bold text-[var(--color-text-primary)] tracking-wide mt-0.5">
                        "Microservices Architecture"
                    </h4>
                </div>

                <div class="flex flex-col lg:flex-row gap-8 items-center lg:items-stretch">
                    // Left/Main column: The diagram
                    <div class="flex-grow w-full flex flex-col gap-6 justify-center">
                        <div class="flex flex-col md:flex-row items-center md:items-stretch gap-4 justify-between">
                            // 1. Client Card
                            <div class="flex-1 w-full flex flex-col justify-center">
                                <TopologyNode
                                    id="client"
                                    icon="🌐"
                                    title="Client Browser"
                                    tech="Rust + Leptos"
                                    hovered_node=hovered_topology_node
                                    set_hovered_node=set_hovered_topology_node
                                />
                            </div>

                            // Arrow Client -> Nginx
                            <div class="flex items-center justify-center text-[var(--color-text-muted)] font-mono text-xs py-1 md:py-0">
                                <span class="md:hidden"><Icon name="arrow-down" /></span>
                                <span class="hidden md:inline text-[var(--color-accent)] opacity-40">
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
                                    hovered_node=hovered_topology_node
                                    set_hovered_node=set_hovered_topology_node
                                />
                            </div>

                            // Arrow Nginx -> K8s Namespace
                            <div class="flex items-center justify-center text-[var(--color-text-muted)] font-mono text-xs py-1 md:py-0">
                                <span class="md:hidden"><Icon name="arrow-down" /></span>
                                <span class="hidden md:inline text-[var(--color-accent)] opacity-40">
                                    "── Proxy ──▶"
                                </span>
                            </div>

                            // 3. Google Cloud Run (Serverless)
                            <div
                                on:mouseenter=move |_| set_hovered_topology_node.set(Some("k8s"))
                                on:mouseleave=move |_| set_hovered_topology_node.set(None)
                                class=k8s_box_class
                            >
                                <div class="flex justify-between items-center mb-3">
                                    <span class="text-[10px] font-bold text-[var(--color-text-muted)] uppercase tracking-widest font-mono">
                                        "Platform: Google Cloud"
                                    </span>
                                    <span class="text-[10px] font-mono text-[var(--color-accent-gold)] bg-[var(--color-accent-bg)] px-1 py-0.5 rounded">
                                        <Icon name="cloud" class="w-3 h-3 mr-1" />"GCP Cloud Run"
                                    </span>
                                </div>

                                <div class="grid grid-cols-3 gap-2">
                                    <TopologyNode
                                        id="frontend"
                                        icon="🦀"
                                        title="Frontend Service"
                                        tech="Cloud Run"
                                        hovered_node=hovered_topology_node
                                        set_hovered_node=set_hovered_topology_node
                                    />
                                    <TopologyNode
                                        id="rust"
                                        icon="🦀"
                                        title="Rust Service"
                                        tech="Cloud Run"
                                        hovered_node=hovered_topology_node
                                        set_hovered_node=set_hovered_topology_node
                                    />
                                    <TopologyNode
                                        id="java"
                                        icon="☕"
                                        title="Java Service"
                                        tech="Cloud Run"
                                        hovered_node=hovered_topology_node
                                        set_hovered_node=set_hovered_topology_node
                                    />
                                </div>
                            </div>
                        </div>

                        // Connections from K8s to external services
                        <div class="flex flex-col md:flex-row gap-4 justify-between items-stretch">
                            // Arrow Rust -> Spotify
                            <div class="flex-1 bg-[var(--color-bg-surface-deep)] border border-[var(--color-border-primary)] rounded-xl p-3.5 flex flex-col justify-between hover:border-[var(--color-accent-orange-border)] transition-all duration-300">
                                <div class="flex justify-between items-center text-[10px] text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
                                    <span>"Rust Gateway"</span>
                                    <span class="text-[var(--color-accent-orange)] font-mono">"Axum Outbound"</span>
                                </div>
                                <div class="flex items-center justify-between gap-2">
                                    <span class="text-xs font-bold text-[var(--color-text-secondary)]">"Rust API"</span>
                                    <span class="text-[var(--color-accent-orange)] font-mono text-[10px]">"── OAuth ──▶"</span>
                                    <TopologyNode
                                        id="spotify"
                                        icon="🎵"
                                        title="Spotify API"
                                        tech="v1 Player Web Service"
                                        hovered_node=hovered_topology_node
                                        set_hovered_node=set_hovered_topology_node
                                    />
                                </div>
                            </div>

                            // Arrow Java -> GitHub
                            <div class="flex-1 bg-[var(--color-bg-surface-deep)] border border-[var(--color-border-primary)] rounded-xl p-3.5 flex flex-col justify-between hover:border-[var(--color-accent-border)] transition-all duration-300">
                                <div class="flex justify-between items-center text-[10px] text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
                                    <span>"Java Caching"</span>
                                    <span class="text-[var(--color-accent)] font-mono">"Spring cacheable"</span>
                                </div>
                                <div class="flex items-center justify-between gap-2">
                                    <span class="text-xs font-bold text-[var(--color-text-secondary)]">"Java API"</span>
                                    <span class="text-[var(--color-accent)] font-mono text-[10px]">"── REST ──▶"</span>
                                    <TopologyNode
                                        id="github"
                                        icon="🐙"
                                        title="GitHub API"
                                        tech="v3 Public REST"
                                        hovered_node=hovered_topology_node
                                        set_hovered_node=set_hovered_topology_node
                                    />
                                </div>
                            </div>
                        </div>
                    </div>

                    // Right/Inspector column: The Topology Inspector
                    <div class="w-full lg:w-80 flex-shrink-0 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] rounded-xl p-5 flex flex-col justify-between relative overflow-hidden">
                        <div class="absolute top-0 right-0 w-24 h-24 bg-[var(--color-accent-bg)] rounded-full blur-2xl -z-10"></div>

                        <div>
                            <div class="flex items-center space-x-2 border-b border-[var(--color-border-primary)] pb-2 mb-3">
                                <span class="flex h-2 w-2 relative">
                                    <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--color-status-online)] opacity-75"></span>
                                    <span class="relative inline-flex rounded-full h-2 w-2 bg-[var(--color-status-online)]"></span>
                                </span>
                                <h4 class="text-xs font-bold text-[var(--color-text-secondary)] uppercase tracking-wider">
                                    "Topology Inspector"
                                </h4>
                            </div>

                            {move || {
                                if let Some(node_id) = hovered_topology_node.get() {
                                    if let Some(info) = get_topology_info(node_id) {
                                        return view! {
                                            <div class="space-y-3.5 animate-fadeIn">
                                                <div class="flex justify-between items-center">
                                                    <span class="text-sm font-bold text-[var(--color-accent)]">
                                                        {info.title}
                                                    </span>
                                                    <span class="bg-[var(--color-accent-bg)] border border-[var(--color-accent-border)] text-[var(--color-accent)] text-[10px] px-1.5 py-0.5 rounded font-mono uppercase tracking-widest">
                                                        {info.badge}
                                                    </span>
                                                </div>

                                                <div class="font-mono text-[10px] space-y-1 bg-[var(--color-bg-surface-inset)] p-2 rounded-lg border border-[var(--color-border-primary)]">
                                                    <div class="flex justify-between">
                                                        <span class="text-[var(--color-text-muted)]">"ENGINE:"</span>
                                                        <span class="text-[var(--color-text-secondary)]">{info.tech}</span>
                                                    </div>
                                                    <div class="flex justify-between mt-1">
                                                        <span class="text-[var(--color-text-muted)]">"PORT/PROTO:"</span>
                                                        <span class="text-[var(--color-accent)] font-semibold">{info.protocol}</span>
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
                                    <div class="text-center py-10 flex flex-col items-center justify-center space-y-3">
                                        <span class="text-2xl opacity-40 animate-bounce"><Icon name="search" /></span>
                                        <p class="text-[var(--color-text-muted)] text-xs leading-relaxed font-mono">
                                            "Hover over any node or container in the diagram to inspect microservice details, proxy routes, and deployment states."
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

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                // Left/Spans 2 columns: GitHub Commits
                <div class="lg:col-span-2 flex flex-col gap-8">
                    <GitHubActivity />
                </div>

                // Right column: Live system telemetry & Spotify
                <div class="lg:col-span-1 flex flex-col gap-6 font-mono">
                    // Rust Engine
                    <section
                        class="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-orange-border)] hover:border-[var(--color-accent-orange)] relative overflow-hidden transition-all duration-300 group"
                        style="box-shadow: var(--shadow-card)"
                    >
                        <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                            <FeedDot health=rust_health accent=DotAccent::Orange />
                            <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-orange-border)] p-2 text-center text-[10px] text-[var(--color-accent-orange)] font-mono shadow-xl z-20">
                                "Live Axum Service"
                            </div>
                        </div>
                        <h3 class="text-lg font-bold text-[var(--color-accent-orange)] mb-0 tracking-wide font-sans">
                            "Rust Engine"
                        </h3>
                        <p class="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">
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
                                    <div class="font-mono text-xs space-y-4 mt-2">
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
                                                    class="h-full bg-gradient-to-r from-[var(--color-accent-orange)] to-[var(--color-accent-amber)] transition-all duration-1000 ease-out shadow-[0_0_8px_var(--color-accent-orange)]"
                                                    style=cpu_width
                                                ></div>
                                            </div>
                                        </div>

                                        <div class="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Threads"</span>
                                            <span class="text-[var(--color-accent-orange)] font-semibold">
                                                {format!("{} Logical Cores", rust.cpu_core_count)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between items-center">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"Memory"</span>
                                            <div class="flex items-center space-x-3">
                                                <Sparkline data=Signal::derive(move || mem_points.clone()) color="var(--color-accent-amber)" max=100.0 />
                                                <span class="text-[var(--color-accent-amber)] font-semibold">
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
                    <section
                        class="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-border)] hover:border-[var(--color-accent)] relative overflow-hidden transition-all duration-300 group"
                        style="box-shadow: var(--shadow-card)"
                    >
                        <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                            <FeedDot health=java_health accent=DotAccent::Online />
                            <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-border)] p-2 text-center text-[10px] text-[var(--color-accent)] font-mono shadow-xl z-20">
                                "Live Spring Service"
                            </div>
                        </div>
                        <h3 class="text-lg font-bold text-[var(--color-accent)] mb-0 tracking-wide font-sans">
                            "Java Infrastructure"
                        </h3>
                        <p class="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">
                            "Spring Cache engine driving GitHub API events"
                        </p>
                        <StaleNotice health=java_health />

                        {move || {
                            if let Some(java) = java_status.get() {
                                view! {
                                    <div class="font-mono text-xs space-y-4 mt-2">
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
                                            <span class="text-[var(--color-accent)] font-semibold">
                                                {format!("{} Threads", java.active_threads)}
                                            </span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-[var(--color-text-muted)] uppercase text-[10px]">"JVM Memory"</span>
                                            <span class="text-[var(--color-accent)] font-semibold">
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
                    <section
                        class="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-border)] hover:border-[var(--color-accent)] relative overflow-hidden transition-all duration-300 group"
                        style="box-shadow: var(--shadow-card)"
                    >
                        <div class="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                            <FeedDot health=network_health accent=DotAccent::Online />
                            <div class="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-border)] p-2 text-center text-[10px] text-[var(--color-accent)] font-mono shadow-xl z-20">
                                "Blackbox Exporter"
                            </div>
                        </div>
                        <h3 class="text-lg font-bold text-[var(--color-accent)] mb-0 tracking-wide font-sans">
                            "Network Telemetry"
                        </h3>
                        <p class="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">
                            "Synthetic latency probes via ICMP (Ping)"
                        </p>
                        <StaleNotice health=network_health />

                        {move || {
                            if let Some(net) = network_status.get() {
                                let google_hist = network_history.get().into_iter().map(|h| h.google_dns).collect::<Vec<_>>();
                                let cloudflare_hist = network_history.get().into_iter().map(|h| h.cloudflare_dns).collect::<Vec<_>>();
                                let riot_hist = network_history.get().into_iter().map(|h| h.riot_games).collect::<Vec<_>>();

                                let google_dot_class = if net.google_dns.status == "online" {
                                    "h-1.5 w-1.5 rounded-full bg-[var(--color-status-online)] shadow-[0_0_6px_var(--color-status-online)]"
                                } else {
                                    "h-1.5 w-1.5 rounded-full bg-[var(--color-status-error)] animate-pulse"
                                };

                                let cf_dot_class = if net.cloudflare_dns.status == "online" {
                                    "h-1.5 w-1.5 rounded-full bg-[var(--color-status-online)] shadow-[0_0_6px_var(--color-status-online)]"
                                } else {
                                    "h-1.5 w-1.5 rounded-full bg-[var(--color-status-error)] animate-pulse"
                                };

                                let riot_dot_class = if net.riot_games.status == "online" {
                                    "h-1.5 w-1.5 rounded-full bg-[var(--color-status-online)] shadow-[0_0_6px_var(--color-status-online)]"
                                } else {
                                    "h-1.5 w-1.5 rounded-full bg-[var(--color-status-error)] animate-pulse"
                                };

                                view! {
                                    <div class="font-mono text-xs space-y-4 mt-2">
                                        // Google DNS
                                        <div class="flex justify-between items-center border-b border-[var(--color-border-primary)] pb-2">
                                            <div class="flex items-center space-x-2">
                                                <span class=google_dot_class></span>
                                                <span class="text-[var(--color-text-primary)]">{net.google_dns.name}</span>
                                                <span class="text-[10px] text-[var(--color-text-muted)]">{format!("({})", net.google_dns.target)}</span>
                                            </div>
                                            <div class="flex items-center space-x-3">
                                                <Sparkline data=Signal::derive(move || google_hist.clone()) color="var(--color-accent)" />
                                                <span class="text-[var(--color-accent)] font-semibold w-14 text-right">
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
                                                <span class="text-[var(--color-accent)] font-semibold w-14 text-right">
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
                                                <span class="text-[var(--color-accent)] font-semibold w-14 text-right">
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
