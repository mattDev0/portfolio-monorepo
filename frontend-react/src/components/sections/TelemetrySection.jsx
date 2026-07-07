import { AlertTriangle } from 'lucide-react';
import Sparkline from '../Sparkline';
import SpotifyPlayer from '../SpotifyPlayer';
import TopologySection from './TopologySection';
import GitHubActivity from '../../GitHubActivity';

const PanelSkeleton = () => (
  <div className="space-y-3.5 mt-2">
    <div className="h-4 w-3/4 skeleton" />
    <div className="h-4 w-5/6 skeleton" />
    <div className="h-4 w-2/3 skeleton" />
  </div>
);

const PanelError = ({ message, onRetry }) => (
  <div className="flex flex-col items-center justify-center py-6 text-center space-y-2">
    <AlertTriangle className="w-5 h-5 text-[var(--status-error)]" />
    <p className="text-xs text-[var(--fg-muted)]">{message}</p>
    <button
      onClick={onRetry || (() => window.location.reload())}
      className="px-2.5 py-1 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] text-[10px] text-[var(--fg-default)] rounded-[var(--radius-sm)] cursor-pointer"
    >
      Retry
    </button>
  </div>
);

export default function TelemetrySection({
  selectedTopologyNode,
  setSelectedTopologyNode,
  rustStatus,
  rustError,
  javaStatus,
  javaError,
  networkStatus,
  networkError,
  telemetryHistory,
  networkHistory,
  spotifyData,
  progressPercent,
  localProgressMs,
  formatTime
}) {
  return (
    <section id="infrastructure" className="space-y-8">
      <div className="border-b border-[var(--border-default)] pb-3">
        <span className="text-[10px] font-bold text-[var(--accent-primary)] uppercase tracking-widest font-mono">
          Systems Observability
        </span>
        <h3 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-bold text-[var(--fg-default)] tracking-wide mt-1">
          Live Infrastructure Dashboard
        </h3>
        <p className="text-xs text-[var(--fg-muted)] mt-1">
          Real-time health telemetry and network architecture mapping my deployed systems.
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        
        {/* Left Column: Topology & GitHub Commits */}
        <div className="lg:col-span-2 flex flex-col gap-8">
          
          <TopologySection
            selectedTopologyNode={selectedTopologyNode}
            setSelectedTopologyNode={setSelectedTopologyNode}
          />
          
          <GitHubActivity />
          
        </div>

        {/* Right Column: Telemetry & Spotify panels */}
        <div className="lg:col-span-1 flex flex-col gap-6 font-mono">
          
          {/* Rust Engine Panel */}
          <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] p-6 rounded-[var(--radius-lg)] relative">
            <div className="absolute top-4 right-4 flex items-center space-x-1.5">
              <span className={`w-2 h-2 rounded-full ${rustError ? 'bg-[var(--status-error)]' : rustStatus ? 'bg-[var(--status-warning)]' : 'bg-[var(--fg-subtle)]'}`} />
              <span className="text-[9px] text-[var(--fg-muted)] font-mono uppercase">
                {rustError ? 'Offline' : rustStatus ? 'Live Service' : 'Handshake'}
              </span>
            </div>

            <h3 className="text-sm font-semibold text-[var(--fg-default)] uppercase tracking-wider mb-1 font-sans">
              Rust Engine
            </h3>
            <p className="text-[10px] text-[var(--fg-subtle)] font-medium mb-5">
              Low-level OS telemetry & Spotify API gateway
            </p>

            {rustError ? (
              <PanelError message="Unable to connect to Rust API." />
            ) : !rustStatus ? (
              <PanelSkeleton />
            ) : (
              <div className="text-xs space-y-4 mt-2">
                <div className="flex justify-between border-b border-[var(--border-muted)] pb-2">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">OS</span>
                  <span className="text-[var(--fg-default)] font-medium">{rustStatus.os_info}</span>
                </div>

                <div className="border-b border-[var(--border-muted)] pb-2">
                  <div className="flex justify-between items-center mb-1.5">
                    <span className="text-[var(--fg-subtle)] uppercase text-[9px]">CPU Utilization</span>
                    <div className="flex items-center space-x-3">
                      <Sparkline data={telemetryHistory.map(h => h.cpu)} color="var(--accent-primary)" max={100} />
                      <span className="text-[var(--fg-default)] font-semibold w-12 text-right">
                        {rustStatus.cpu_usage_percent !== undefined ? `${rustStatus.cpu_usage_percent.toFixed(1)}%` : "0%"}
                      </span>
                    </div>
                  </div>
                  <div className="w-full h-1 bg-[var(--bg-inset)] rounded-full overflow-hidden">
                    <div
                      className="h-full bg-[var(--accent-primary)] transition-all duration-1000 ease-out"
                      style={{ width: `${rustStatus.cpu_usage_percent || 0}%` }}
                    ></div>
                  </div>
                </div>

                <div className="flex justify-between border-b border-[var(--border-muted)] pb-2">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">Threads</span>
                  <span className="text-[var(--fg-default)] font-semibold">
                    {rustStatus.cpu_core_count} Logical Cores
                  </span>
                </div>

                <div className="flex justify-between items-center">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">Memory</span>
                  <div className="flex items-center space-x-3">
                    <Sparkline data={telemetryHistory.map(h => h.memory)} color="var(--accent-primary)" max={100} />
                    <span className="text-[var(--fg-default)] font-semibold w-18 text-right">
                      {rustStatus.memory_used_mb} MB / {rustStatus.memory_total_mb} MB
                    </span>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Java Infrastructure Panel */}
          <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] p-6 rounded-[var(--radius-lg)] relative">
            <div className="absolute top-4 right-4 flex items-center space-x-1.5">
              <span className={`w-2 h-2 rounded-full ${javaError ? 'bg-[var(--status-error)]' : javaStatus ? 'bg-[var(--status-success)]' : 'bg-[var(--fg-subtle)]'}`} />
              <span className="text-[9px] text-[var(--fg-muted)] font-mono uppercase">
                {javaError ? 'Offline' : javaStatus ? 'Live Service' : 'Handshake'}
              </span>
            </div>

            <h3 className="text-sm font-semibold text-[var(--fg-default)] uppercase tracking-wider mb-1 font-sans">
              Java Infrastructure
            </h3>
            <p className="text-[10px] text-[var(--fg-subtle)] font-medium mb-5">
              Spring Cache engine driving GitHub API events
            </p>

            {javaError ? (
              <PanelError message="Unable to connect to Java API." />
            ) : !javaStatus ? (
              <PanelSkeleton />
            ) : (
              <div className="text-xs space-y-4 mt-2">
                <div className="flex justify-between border-b border-[var(--border-muted)] pb-2">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">Version</span>
                  <span className="text-[var(--fg-default)] font-medium">{javaStatus.engine}</span>
                </div>
                <div className="flex justify-between border-b border-[var(--border-muted)] pb-2">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">Uptime</span>
                  <span className="text-[var(--fg-default)] font-semibold">
                    {javaStatus.uptime_hours}h {javaStatus.uptime_minutes}m
                  </span>
                </div>
                <div className="flex justify-between border-b border-[var(--border-muted)] pb-2">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">Active Threads</span>
                  <span className="text-[var(--fg-default)] font-semibold">
                    {javaStatus.active_threads} Threads
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-[var(--fg-subtle)] uppercase text-[9px]">JVM Memory</span>
                  <span className="text-[var(--fg-default)] font-semibold">
                    {javaStatus.jvm_memory_used_mb} MB / {javaStatus.jvm_memory_total_mb} MB
                  </span>
                </div>
              </div>
            )}
          </div>

          {/* Network Telemetry Panel */}
          <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] p-6 rounded-[var(--radius-lg)] relative">
            <div className="absolute top-4 right-4 flex items-center space-x-1.5">
              <span className={`w-2 h-2 rounded-full ${networkError ? 'bg-[var(--status-error)]' : networkStatus ? 'bg-[var(--status-success)]' : 'bg-[var(--fg-subtle)]'}`} />
              <span className="text-[9px] text-[var(--fg-muted)] font-mono uppercase">
                {networkError ? 'Offline' : networkStatus ? 'Live Probes' : 'Handshake'}
              </span>
            </div>

            <h3 className="text-sm font-semibold text-[var(--fg-default)] uppercase tracking-wider mb-1 font-sans">
              Network Telemetry
            </h3>
            <p className="text-[10px] text-[var(--fg-subtle)] font-medium mb-5">
              Synthetic latency probes via ICMP (Ping)
            </p>

            {networkError ? (
              <PanelError message="Unable to connect to Network API." />
            ) : !networkStatus ? (
              <PanelSkeleton />
            ) : (
              <div className="text-xs space-y-4 mt-2">
                {/* Google DNS */}
                <div className="flex justify-between items-center border-b border-[var(--border-muted)] pb-2">
                  <div className="flex items-center space-x-2">
                    <span className={`w-1.5 h-1.5 rounded-full ${networkStatus.google_dns.status === 'online' ? 'bg-[var(--status-success)]' : 'bg-[var(--status-error)]'}`} />
                    <span className="text-[var(--fg-default)]">{networkStatus.google_dns.name}</span>
                    <span className="text-[9px] text-[var(--fg-subtle)]">({networkStatus.google_dns.target})</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <Sparkline data={networkHistory.map(h => h.google_dns)} color="var(--accent-primary)" />
                    <span className="text-[var(--fg-default)] font-semibold w-14 text-right">
                      {networkStatus.google_dns.latency_ms} ms
                    </span>
                  </div>
                </div>

                {/* Cloudflare DNS */}
                <div className="flex justify-between items-center border-b border-[var(--border-muted)] pb-2">
                  <div className="flex items-center space-x-2">
                    <span className={`w-1.5 h-1.5 rounded-full ${networkStatus.cloudflare_dns.status === 'online' ? 'bg-[var(--status-success)]' : 'bg-[var(--status-error)]'}`} />
                    <span className="text-[var(--fg-default)]">{networkStatus.cloudflare_dns.name}</span>
                    <span className="text-[9px] text-[var(--fg-subtle)]">({networkStatus.cloudflare_dns.target})</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <Sparkline data={networkHistory.map(h => h.cloudflare_dns)} color="var(--accent-primary)" />
                    <span className="text-[var(--fg-default)] font-semibold w-14 text-right">
                      {networkStatus.cloudflare_dns.latency_ms} ms
                    </span>
                  </div>
                </div>

                {/* Riot Games NA */}
                <div className="flex justify-between items-center">
                  <div className="flex items-center space-x-2">
                    <span className={`w-1.5 h-1.5 rounded-full ${networkStatus.riot_games.status === 'online' ? 'bg-[var(--status-success)]' : 'bg-[var(--status-error)]'}`} />
                    <span className="text-[var(--fg-default)]">{networkStatus.riot_games.name}</span>
                    <span className="text-[9px] text-[var(--fg-subtle)]">({networkStatus.riot_games.target})</span>
                  </div>
                  <div className="flex items-center space-x-3">
                    <Sparkline data={networkHistory.map(h => h.riot_games)} color="var(--accent-primary)" />
                    <span className="text-[var(--fg-default)] font-semibold w-14 text-right">
                      {networkStatus.riot_games.latency_ms} ms
                    </span>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Spotify Player */}
          <SpotifyPlayer
            spotifyData={spotifyData}
            progressPercent={progressPercent}
            localProgressMs={localProgressMs}
            formatTime={formatTime}
          />

        </div>

      </div>
    </section>
  );
}
