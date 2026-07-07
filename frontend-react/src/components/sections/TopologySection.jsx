import { TOPOLOGY_INFO } from '../../constants';
import TopologyNode from '../TopologyNode';

export default function TopologySection({ selectedTopologyNode, setSelectedTopologyNode }) {
  return (
    <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-6 md:p-8">
      <div className="border-b border-[var(--border-default)] pb-3 mb-6">
        <span className="text-[10px] font-bold text-[var(--accent-primary)] uppercase tracking-widest font-mono">
          Deployment Topology
        </span>
        <h2 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-semibold text-[var(--fg-default)] mt-0.5">
          Microservices Architecture
        </h2>
      </div>

      {/* Topology diagram wrapper */}
      <div className="flex flex-col lg:flex-row gap-6 items-stretch">

        {/* Left/Main column: The diagram */}
        <div className="flex-grow w-full max-w-3xl flex flex-col gap-6 justify-center">

          <div className="flex flex-col md:flex-row items-center md:items-stretch gap-4 justify-between">

            {/* 1. Client Card */}
            <div className="flex-1 w-full flex flex-col justify-center">
              <TopologyNode
                id="client"
                title="Client Browser"
                tech="React 19 + Vite"
                isSelected={selectedTopologyNode === 'client'}
                onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'client' ? null : 'client')}
              />
            </div>

            {/* Arrow Client -> Nginx */}
            <div className="flex items-center justify-center text-[var(--fg-subtle)] font-mono text-[10px] py-1 md:py-0 shrink-0 select-none">
              <span className="md:hidden">↓</span>
              <span className="hidden md:inline">── HTTPS ──&gt;</span>
            </div>

            {/* 2. Nginx Card */}
            <div className="flex-1 w-full flex flex-col justify-center">
              <TopologyNode
                id="nginx"
                title="Nginx Proxy"
                tech="SSL Gateway"
                isSelected={selectedTopologyNode === 'nginx'}
                onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'nginx' ? null : 'nginx')}
              />
            </div>

            {/* Arrow Nginx -> K8s Namespace */}
            <div className="flex items-center justify-center text-[var(--fg-subtle)] font-mono text-[10px] py-1 md:py-0 shrink-0 select-none">
              <span className="md:hidden">↓</span>
              <span className="hidden md:inline">── PROXY ──&gt;</span>
            </div>

            {/* 3. K3s Kubernetes Namespace container */}
            <div className={`flex-[2] w-full p-4 rounded-[var(--radius-lg)] border border-dashed transition-colors duration-150 ${
              selectedTopologyNode === 'k8s'
                ? 'border-[var(--accent-primary)] bg-[var(--bg-elevated)]'
                : 'border-[var(--border-default)] bg-[var(--bg-surface)]'
            }`}>
              <div className="flex justify-between items-center mb-3">
                <button
                  onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'k8s' ? null : 'k8s')}
                  aria-pressed={selectedTopologyNode === 'k8s'}
                  className="text-[9px] font-bold text-[var(--fg-muted)] hover:text-[var(--fg-default)] uppercase tracking-wider font-mono cursor-pointer focus-visible:outline-none"
                >
                  K8s Namespace: portfolio
                </button>
                <span className="text-[8px] font-mono text-[var(--accent-primary)] bg-[var(--bg-inset)] px-1.5 py-0.5 rounded border border-[var(--border-default)]">
                  K3s Cluster
                </span>
              </div>

              <div className="grid grid-cols-3 gap-2">
                {/* Pod 1: Frontend */}
                <TopologyNode
                  id="frontend"
                  title="Frontend Pod"
                  tech="Static Nginx"
                  isSelected={selectedTopologyNode === 'frontend'}
                  onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'frontend' ? null : 'frontend')}
                />
                {/* Pod 2: Rust API */}
                <TopologyNode
                  id="rust"
                  title="Rust Pod"
                  tech="Axum Engine"
                  isSelected={selectedTopologyNode === 'rust'}
                  onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'rust' ? null : 'rust')}
                />
                {/* Pod 3: Java API */}
                <TopologyNode
                  id="java"
                  title="Java Pod"
                  tech="Spring Boot"
                  isSelected={selectedTopologyNode === 'java'}
                  onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'java' ? null : 'java')}
                />
              </div>
            </div>

          </div>

          {/* Connections from K8s to external services */}
          <div className="flex flex-col md:flex-row gap-4 justify-between items-stretch">

            {/* Outbound connection: Rust -> Spotify */}
            <div className="flex-1 bg-[var(--bg-inset)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-4 flex flex-col justify-between">
              <div className="flex justify-between items-center text-[8px] text-[var(--fg-subtle)] uppercase tracking-wider mb-3">
                <span>Rust Gateway</span>
                <span className="text-[var(--accent-primary)] font-mono">Axum Outbound</span>
              </div>
              <div className="flex items-center justify-between gap-3">
                <span className="text-xs font-semibold text-[var(--fg-default)]">Rust API</span>
                <span className="text-[var(--fg-subtle)] font-mono text-[10px] select-none">── OAuth ──&gt;</span>
                <div className="w-28 shrink-0">
                  <TopologyNode
                    id="spotify"
                    title="Spotify API"
                    tech="Web Services"
                    isSelected={selectedTopologyNode === 'spotify'}
                    onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'spotify' ? null : 'spotify')}
                  />
                </div>
              </div>
            </div>

            {/* Outbound connection: Java -> GitHub */}
            <div className="flex-1 bg-[var(--bg-inset)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-4 flex flex-col justify-between">
              <div className="flex justify-between items-center text-[8px] text-[var(--fg-subtle)] uppercase tracking-wider mb-3">
                <span>Java Caching</span>
                <span className="text-[var(--accent-primary)] font-mono">Spring Cache</span>
              </div>
              <div className="flex items-center justify-between gap-3">
                <span className="text-xs font-semibold text-[var(--fg-default)]">Java API</span>
                <span className="text-[var(--fg-subtle)] font-mono text-[10px] select-none">── REST ──&gt;</span>
                <div className="w-28 shrink-0">
                  <TopologyNode
                    id="github"
                    title="GitHub API"
                    tech="REST v3"
                    isSelected={selectedTopologyNode === 'github'}
                    onClick={() => setSelectedTopologyNode(selectedTopologyNode === 'github' ? null : 'github')}
                  />
                </div>
              </div>
            </div>

          </div>

        </div>

        {/* Right/Inspector column: The Topology Inspector */}
        <div className="w-full lg:w-80 flex-shrink-0 bg-[var(--bg-inset)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-5 flex flex-col justify-between">
          <div>
            <div className="flex items-center space-x-2 border-b border-[var(--border-default)] pb-2 mb-3">
              <span className="flex h-2 w-2 relative">
                <span className="inline-flex rounded-full h-2 w-2 bg-[var(--accent-primary)]"></span>
              </span>
              <h3 className="text-xs font-bold text-[var(--fg-muted)] uppercase tracking-wider">
                Topology Inspector
              </h3>
            </div>

            {selectedTopologyNode ? (
              <div className="space-y-3.5">
                <div className="flex justify-between items-center">
                  <span className="text-sm font-bold text-[var(--accent-primary)]">
                    {TOPOLOGY_INFO[selectedTopologyNode].title}
                  </span>
                  <span className="bg-[var(--bg-elevated)] border border-[var(--border-default)] text-[var(--fg-muted)] text-[8px] px-1.5 py-0.5 rounded font-mono uppercase tracking-widest">
                    {TOPOLOGY_INFO[selectedTopologyNode].badge}
                  </span>
                </div>

                <div className="font-mono text-[10px] space-y-1 bg-[var(--bg-surface)] p-2 rounded-lg border border-[var(--border-default)]">
                  <div className="flex justify-between">
                    <span className="text-[var(--fg-subtle)]">ENGINE:</span>
                    <span className="text-[var(--fg-default)]">{TOPOLOGY_INFO[selectedTopologyNode].tech}</span>
                  </div>
                  <div className="flex justify-between mt-1">
                    <span className="text-[var(--fg-subtle)]">PORT/PROTO:</span>
                    <span className="text-[var(--accent-primary)] font-semibold">{TOPOLOGY_INFO[selectedTopologyNode].protocol}</span>
                  </div>
                </div>

                <p className="text-[var(--text-sm)] leading-relaxed text-[var(--fg-muted)]">
                  {TOPOLOGY_INFO[selectedTopologyNode].description}
                </p>
              </div>
            ) : (
              <div className="text-center py-12 flex flex-col items-center justify-center space-y-3">
                <p className="text-[var(--fg-subtle)] text-xs leading-normal font-mono">
                  Select any node or container in the diagram to inspect microservice details, proxy routes, and deployment states.
                </p>
              </div>
            )}
          </div>

          <div className="mt-6 pt-3 border-t border-[var(--border-default)] text-[9px] text-[var(--fg-subtle)] font-mono flex items-center justify-between">
            <span>DEPLOYMENT: live</span>
            <span>VM: azure-standard-b1s</span>
          </div>
        </div>

      </div>
    </div>
  );
}
