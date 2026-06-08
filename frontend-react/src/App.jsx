import { useState, useEffect } from 'react';
import portfolioConfig from './config.json';
import GitHubActivity from './GitHubActivity';
import { apiUrl } from './api';

// Components
import Sparkline from './components/Sparkline';
import TerminalSimulator from './components/TerminalSimulator';
import TopologyNode from './components/TopologyNode';
import SpotifyPlayer from './components/SpotifyPlayer';

// Hooks
import useTelemetry from './hooks/useTelemetry';
import useSpotify from './hooks/useSpotify';

// Constants
import { TOPOLOGY_INFO } from './constants';

function App() {
  const [selectedTech, setSelectedTech] = useState(null);
  const [showDevOpsCaseStudy, setShowDevOpsCaseStudy] = useState(false);
  const [hoveredTopologyNode, setHoveredTopologyNode] = useState(null);
  const [isVisible, setIsVisible] = useState(true);

  // Monitor tab/window visibility state
  useEffect(() => {
    const handleVisibilityChange = () => {
      setIsVisible(document.visibilityState === 'visible');
    };
    document.addEventListener('visibilitychange', handleVisibilityChange);
    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  }, []);

  const {
    rustStatus,
    javaStatus,
    telemetryHistory,
    networkStatus,
    networkHistory,
    getCpuPercentage
  } = useTelemetry(isVisible);

  const {
    spotifyData,
    localProgressMs,
    progressPercent,
    formatTime
  } = useSpotify(isVisible);

  // Extract all unique technology tags from projects
  const allTechTags = Array.from(
    new Set(portfolioConfig.projects.flatMap(p => p.tech))
  );

  const filteredProjects = selectedTech
    ? portfolioConfig.projects.filter(p => p.tech.includes(selectedTech))
    : portfolioConfig.projects;

  return (
    <div className="min-h-screen bg-[#0b0f19] text-gray-100 flex flex-col items-center py-16 px-4 sm:px-8 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-blue-950/20 via-[#0b0f19] to-[#0b0f19]">
      
      {/* Header Section */}
      <header className="text-center mb-16 max-w-3xl w-full">
        <h1 className="text-5xl md:text-6xl font-extrabold bg-gradient-to-r from-blue-400 via-indigo-400 to-purple-400 bg-clip-text text-transparent mb-4 tracking-tight">
          {portfolioConfig.name}
        </h1>
        <h2 className="text-2xl font-semibold text-gray-300 mb-4 tracking-wide">{portfolioConfig.title}</h2>
        
        {/* Location and Remote Status */}
        <div className="flex items-center justify-center text-gray-400 mb-6 text-sm">
          <svg className="w-4 h-4 mr-2 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span className="font-medium text-gray-300">{portfolioConfig.location}</span>
          
          {portfolioConfig.openToRemote && (
            <>
              <span className="mx-3 text-gray-700">•</span>
              <span className="bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded-full text-xs font-semibold tracking-wider uppercase">
                Open to Remote
              </span>
            </>
          )}
        </div>

        {/* Tagline */}
        <p className="max-w-2xl mx-auto text-gray-400 leading-relaxed text-base italic mb-8">
          "{portfolioConfig.tagline}"
        </p>

        {/* Contact Links */}
        <div className="flex items-center justify-center space-x-4">
          <a 
            href={`mailto:${portfolioConfig.email}`} 
            className="flex items-center space-x-2 px-6 py-2.5 bg-white/5 border border-white/10 text-gray-300 hover:text-white hover:bg-white/10 hover:border-white/20 rounded-xl transition-all duration-300 shadow-md"
          >
            <span>📧</span>
            <span className="font-semibold text-sm">Email Me</span>
          </a>
          <a 
            href={portfolioConfig.githubUrl} 
            target="_blank" 
            rel="noopener noreferrer" 
            className="flex items-center space-x-2 px-6 py-2.5 bg-blue-600/10 border border-blue-500/20 text-blue-400 hover:bg-blue-600/20 hover:text-blue-300 hover:border-blue-500/40 rounded-xl transition-all duration-300 shadow-md shadow-blue-900/5"
          >
            <span>🐙</span>
            <span className="font-semibold text-sm">GitHub</span>
          </a>
        </div>
      </header>
      
      <main className="w-full max-w-5xl space-y-12">
        
        {/* Profile, Experience & Telemetry Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          
          {/* --- LEFT COLUMN: About, Experience & GitHub Activity (Spans 2 of 3 columns) --- */}
          <div className="lg:col-span-2 flex flex-col gap-8">
            
            {/* About the Developer */}
            <section className="bg-slate-900/30 backdrop-blur-md p-6 md:p-8 rounded-2xl shadow-xl border border-white/5 flex flex-col justify-between hover:border-white/10 transition-colors duration-300">
              <div>
                <h3 className="text-xl font-bold text-white mb-4 border-b border-white/5 pb-3 tracking-wide">About the Developer</h3>
                <p className="text-gray-300 leading-relaxed text-sm">
                  {portfolioConfig.about}
                </p>
              </div>
              <div className="mt-8 pt-6 border-t border-white/5">
                 <h4 className="text-xs font-bold text-indigo-400 uppercase tracking-widest mb-2">Education</h4>
                 <div className="flex justify-between items-center">
                   <span className="text-gray-200 font-semibold">{portfolioConfig.education.degree}</span>
                   <span className="text-indigo-400 font-semibold text-sm">{portfolioConfig.education.year}</span>
                 </div>
                 <div className="text-gray-400 text-sm mt-1">{portfolioConfig.education.institution}</div>
              </div>
            </section>
 
            {/* Nested Grid for Experience & GitHub Activity */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              {/* Recent Roles (Experience) */}
              <section className="bg-slate-900/30 backdrop-blur-md p-6 md:p-8 rounded-2xl shadow-xl border border-white/5 hover:border-white/10 transition-colors duration-300">
                <h3 className="text-xl font-bold text-white mb-6 border-b border-white/5 pb-3 tracking-wide">Recent Roles</h3>
                <div className="space-y-8">
                  {portfolioConfig.experience.map((exp, index) => (
                    <div key={index} className="relative pl-6 border-l-2 border-indigo-500/30 hover:border-indigo-400 transition-colors duration-300">
                      <div className="absolute w-3 h-3 bg-indigo-500 rounded-full -left-[7px] top-1.5 border border-slate-900 shadow-[0_0_8px_#6366f1]"></div>
                      <h4 className="text-gray-100 font-bold text-sm tracking-wide">{exp.role}</h4>
                      <p className="text-indigo-400 text-xs font-medium mb-2">{exp.company}</p>
                      <p className="text-gray-400 text-xs leading-relaxed">{exp.description}</p>
                    </div>
                  ))}
                </div>
              </section>

              {/* Live GitHub Activity */}
              <GitHubActivity />
            </div>
            
          </div>
          {/* --- END LEFT COLUMN --- */}
 
          {/* --- RIGHT COLUMN: Telemetry Stack (Spans 1 of 3 columns) --- */}
          <div className="lg:col-span-1 flex flex-col gap-6">
            <h3 className="text-xl font-bold text-white mb-2 border-b border-white/5 pb-3 tracking-wide">Live Systems & Telemetry</h3>
            
            {/* Rust Engine */}
            <section className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-orange-500/10 hover:border-orange-500/30 relative overflow-hidden transition-all duration-300 group hover:shadow-lg hover:shadow-orange-950/5">
              <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400/40 opacity-40 delay-300"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-orange-500 border border-slate-900 shadow-[0_0_8px_#f97316]"></span>
                </span>
                {/* Custom Tooltip */}
                <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-slate-950/95 border border-orange-500/20 p-2 text-center text-[10px] text-orange-400 font-mono shadow-xl z-20">
                  Live Axum Service
                </div>
              </div>
              <h3 className="text-lg font-bold text-orange-400 mb-0 tracking-wide">Rust Engine</h3>
              <p className="text-[10px] text-gray-500 font-medium mb-5">Low-level OS telemetry & Spotify API gateway</p>
              
              {rustStatus ? (
                <div className="font-mono text-xs space-y-4 mt-2">
                  <div className="flex justify-between border-b border-white/5 pb-2">
                    <span className="text-gray-500 uppercase text-[10px]">OS</span>
                    <span className="text-gray-200 font-medium">{rustStatus.os_info}</span>
                  </div>
                  
                  {/* Real-time CPU Usage bar */}
                  <div className="border-b border-white/5 pb-2">
                    <div className="flex justify-between items-center mb-1.5">
                      <span className="text-gray-500 uppercase text-[10px]">CPU Utilization</span>
                      <div className="flex items-center space-x-3">
                        <Sparkline data={telemetryHistory.map(h => h.cpu)} color="#f97316" max={100} />
                        <span className="text-orange-400 font-semibold">{rustStatus.cpu_usage || "0%"}</span>
                      </div>
                    </div>
                    <div className="w-full h-1.5 bg-slate-950 rounded-full overflow-hidden">
                      <div 
                        className="h-full bg-gradient-to-r from-orange-500 to-amber-400 transition-all duration-1000 ease-out shadow-[0_0_8px_#f97316]"
                        style={{ width: `${getCpuPercentage(rustStatus.cpu_usage)}%` }}
                      ></div>
                    </div>
                  </div>

                  <div className="flex justify-between border-b border-white/5 pb-2">
                    <span className="text-gray-500 uppercase text-[10px]">Threads</span>
                    <span className="text-orange-400 font-semibold">{rustStatus.cpu_cores}</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-gray-500 uppercase text-[10px]">Memory</span>
                    <div className="flex items-center space-x-3">
                      <Sparkline data={telemetryHistory.map(h => h.memory)} color="#f59e0b" max={100} />
                      <span className="text-amber-500 font-semibold">{rustStatus.memory_usage}</span>
                    </div>
                  </div>
                </div>
              ) : (
                <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Handshake pending...</p>
              )}
            </section>
 
            {/* Java Infrastructure */}
            <section className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-emerald-500/10 hover:border-emerald-500/30 relative overflow-hidden transition-all duration-300 group hover:shadow-lg hover:shadow-emerald-950/5">
              <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400/40 opacity-40 delay-300"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500 shadow-[0_0_8px_#10b981]"></span>
                </span>
                {/* Custom Tooltip */}
                <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-slate-950/95 border border-emerald-500/20 p-2 text-center text-[10px] text-emerald-400 font-mono shadow-xl z-20">
                  Live Spring Service
                </div>
              </div>
              <h3 className="text-lg font-bold text-emerald-400 mb-0 tracking-wide">Java Infrastructure</h3>
              <p className="text-[10px] text-gray-500 font-medium mb-5">Spring Cache engine driving GitHub API events</p>
              
              {javaStatus ? (
                <div className="font-mono text-xs space-y-4 mt-2">
                  <div className="flex justify-between border-b border-white/5 pb-2">
                    <span className="text-gray-500 uppercase text-[10px]">Version</span>
                    <span className="text-gray-200 font-medium">{javaStatus.engine}</span>
                  </div>
                  <div className="flex justify-between border-b border-white/5 pb-2">
                    <span className="text-gray-500 uppercase text-[10px]">Uptime</span>
                    <span className="text-emerald-400 font-semibold">{javaStatus.uptime}</span>
                  </div>
                  <div className="flex justify-between border-b border-white/5 pb-2">
                    <span className="text-gray-500 uppercase text-[10px]">Active Threads</span>
                    <span className="text-emerald-400 font-semibold">{javaStatus.active_threads}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500 uppercase text-[10px]">JVM Memory</span>
                    <span className="text-emerald-500 font-semibold">{javaStatus.jvm_memory}</span>
                  </div>
                </div>
              ) : (
                <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Handshake pending...</p>
              )}
            </section>
 
            {/* Network Telemetry */}
            <section className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-indigo-500/10 hover:border-indigo-500/30 relative overflow-hidden transition-all duration-300 group hover:shadow-lg hover:shadow-indigo-950/5">
              <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75"></span>
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400/40 opacity-40 delay-300"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-indigo-500 shadow-[0_0_8px_#6366f1]"></span>
                </span>
                {/* Custom Tooltip */}
                <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-slate-950/95 border border-indigo-500/20 p-2 text-center text-[10px] text-indigo-400 font-mono shadow-xl z-20">
                  Blackbox Exporter
                </div>
              </div>
              <h3 className="text-lg font-bold text-indigo-400 mb-0 tracking-wide">Network Telemetry</h3>
              <p className="text-[10px] text-gray-500 font-medium mb-5">Synthetic latency probes via ICMP (Ping)</p>

              {networkStatus ? (
                <div className="font-mono text-xs space-y-4 mt-2">
                  {/* Google DNS */}
                  <div className="flex justify-between items-center border-b border-white/5 pb-2">
                    <div className="flex items-center space-x-2">
                      <span className={`h-1.5 w-1.5 rounded-full ${networkStatus.google_dns.status === 'online' ? 'bg-emerald-500 shadow-[0_0_6px_#10b981]' : 'bg-rose-500 animate-pulse'}`}></span>
                      <span className="text-gray-200">{networkStatus.google_dns.name}</span>
                      <span className="text-[9px] text-gray-500">({networkStatus.google_dns.target})</span>
                    </div>
                    <div className="flex items-center space-x-3">
                      <Sparkline data={networkHistory.map(h => h.google_dns)} color="#6366f1" />
                      <span className="text-indigo-400 font-semibold w-14 text-right">{networkStatus.google_dns.latency_ms} ms</span>
                    </div>
                  </div>

                  {/* Cloudflare DNS */}
                  <div className="flex justify-between items-center border-b border-white/5 pb-2">
                    <div className="flex items-center space-x-2">
                      <span className={`h-1.5 w-1.5 rounded-full ${networkStatus.cloudflare_dns.status === 'online' ? 'bg-emerald-500 shadow-[0_0_6px_#10b981]' : 'bg-rose-500 animate-pulse'}`}></span>
                      <span className="text-gray-200">{networkStatus.cloudflare_dns.name}</span>
                      <span className="text-[9px] text-gray-500">({networkStatus.cloudflare_dns.target})</span>
                    </div>
                    <div className="flex items-center space-x-3">
                      <Sparkline data={networkHistory.map(h => h.cloudflare_dns)} color="#6366f1" />
                      <span className="text-indigo-400 font-semibold w-14 text-right">{networkStatus.cloudflare_dns.latency_ms} ms</span>
                    </div>
                  </div>

                  {/* Riot Games NA */}
                  <div className="flex justify-between items-center">
                    <div className="flex items-center space-x-2">
                      <span className={`h-1.5 w-1.5 rounded-full ${networkStatus.riot_games.status === 'online' ? 'bg-emerald-500 shadow-[0_0_6px_#10b981]' : 'bg-rose-500 animate-pulse'}`}></span>
                      <span className="text-gray-200">{networkStatus.riot_games.name}</span>
                      <span className="text-[9px] text-gray-500">({networkStatus.riot_games.target})</span>
                    </div>
                    <div className="flex items-center space-x-3">
                      <Sparkline data={networkHistory.map(h => h.riot_games)} color="#6366f1" />
                      <span className="text-indigo-400 font-semibold w-14 text-right">{networkStatus.riot_games.latency_ms} ms</span>
                    </div>
                  </div>
                </div>
              ) : (
                <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Ping diagnostics pending...</p>
              )}
            </section>

            {/* Live Spotify Session via Rust */}
            <SpotifyPlayer
              spotifyData={spotifyData}
              progressPercent={progressPercent}
              localProgressMs={localProgressMs}
              formatTime={formatTime}
            />
            
          </div>
          {/* --- END RIGHT COLUMN --- */}
          
        </div>

        {/* System Architecture & Topology Diagram Section */}
        <section className="bg-slate-900/30 backdrop-blur-md p-6 md:p-8 rounded-2xl border border-white/5 hover:border-white/10 transition-colors duration-300">
          <div className="border-b border-white/5 pb-3 mb-6">
            <span className="text-[10px] font-bold text-indigo-400 uppercase tracking-widest">Observability & System Design</span>
            <h3 className="text-2xl font-extrabold text-white tracking-wide mt-1">Production System Architecture</h3>
            <p className="text-xs text-gray-400 mt-1">Interactive layout mapping the client-to-cloud microservices network running on Azure. Hover over any node to inspect.</p>
          </div>

          {/* Topology diagram wrapper */}
          <div className="flex flex-col lg:flex-row gap-8 items-center lg:items-stretch">
            
            {/* Left/Main column: The diagram */}
            <div className="flex-grow w-full max-w-3xl flex flex-col gap-6 justify-center">
              
              <div className="flex flex-col md:flex-row items-center md:items-stretch gap-4 justify-between">
                
                {/* 1. Client Card */}
                <div className="flex-1 w-full flex flex-col justify-center">
                  <TopologyNode id="client" icon="🌐" title="Client Browser" tech="React 19 + Vite" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                </div>

                {/* Arrow Client -> Nginx */}
                <div className="flex items-center justify-center text-gray-700 font-mono text-xs py-1 md:py-0">
                  <span className="md:hidden">⬇️</span>
                  <span className="hidden md:inline text-indigo-400/40">── HTTPS ──▶</span>
                </div>

                {/* 2. Nginx Card */}
                <div className="flex-1 w-full flex flex-col justify-center">
                  <TopologyNode id="nginx" icon="🛡️" title="Nginx Proxy" tech="Port 443 SSL" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                </div>

                {/* Arrow Nginx -> K8s Namespace */}
                <div className="flex items-center justify-center text-gray-700 font-mono text-xs py-1 md:py-0">
                  <span className="md:hidden">⬇️</span>
                  <span className="hidden md:inline text-indigo-400/40">── Proxy ──▶</span>
                </div>

                {/* 3. K3s Kubernetes Namespace (Acts as a container for internal pods) */}
                <div 
                  onMouseEnter={() => setHoveredTopologyNode('k8s')}
                  onMouseLeave={() => setHoveredTopologyNode(null)}
                  className={`flex-[2] w-full p-4 rounded-xl border border-dashed transition-all duration-300 ${hoveredTopologyNode === 'k8s' ? 'border-blue-400 bg-blue-500/2 shadow-[0_0_15px_rgba(96,165,250,0.1)]' : 'border-white/10 bg-slate-950/10'}`}
                >
                  <div className="flex justify-between items-center mb-3">
                    <span className="text-[8px] font-bold text-gray-500 uppercase tracking-widest">K8s Namespace: portfolio</span>
                    <span className="text-[8px] font-mono text-blue-400 bg-blue-950/40 px-1 py-0.5 rounded">☸️ K3s Cluster</span>
                  </div>

                  <div className="grid grid-cols-3 gap-2">
                    {/* Pod 1: Frontend */}
                    <TopologyNode id="frontend" icon="⚛️" title="Frontend Pod" tech="NodePort 30000" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                    {/* Pod 2: Rust API */}
                    <TopologyNode id="rust" icon="🦀" title="Rust Pod" tech="NodePort 30080" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                    {/* Pod 3: Java API */}
                    <TopologyNode id="java" icon="☕" title="Java Pod" tech="NodePort 30081" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                  </div>
                </div>

              </div>

              {/* Connections from K8s to external services (Horizontal flow below) */}
              <div className="flex flex-col md:flex-row gap-4 justify-between items-stretch">
                
                {/* Arrow Rust -> Spotify */}
                <div className="flex-1 bg-slate-950/20 border border-white/5 rounded-xl p-3.5 flex flex-col justify-between hover:border-orange-500/30 transition-all duration-300">
                  <div className="flex justify-between items-center text-[8px] text-gray-500 uppercase tracking-wider mb-2">
                    <span>Rust Gateway</span>
                    <span className="text-orange-400 font-mono">Axum Outbound</span>
                  </div>
                  <div className="flex items-center justify-between gap-2">
                    <span className="text-xs font-bold text-gray-300">Rust API</span>
                    <span className="text-orange-400 font-mono text-[10px]">── OAuth ──▶</span>
                    <TopologyNode id="spotify" icon="🎵" title="Spotify API" tech="v1 Player Web Service" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                  </div>
                </div>

                {/* Arrow Java -> GitHub */}
                <div className="flex-1 bg-slate-950/20 border border-white/5 rounded-xl p-3.5 flex flex-col justify-between hover:border-emerald-500/30 transition-all duration-300">
                  <div className="flex justify-between items-center text-[8px] text-gray-500 uppercase tracking-wider mb-2">
                    <span>Java Caching</span>
                    <span className="text-emerald-400 font-mono">Spring cacheable</span>
                  </div>
                  <div className="flex items-center justify-between gap-2">
                    <span className="text-xs font-bold text-gray-300">Java API</span>
                    <span className="text-emerald-400 font-mono text-[10px]">── REST ──▶</span>
                    <TopologyNode id="github" icon="🐙" title="GitHub API" tech="v3 Public REST" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                  </div>
                </div>

              </div>

            </div>

            {/* Right/Inspector column: The Topology Inspector */}
            <div className="w-full lg:w-80 flex-shrink-0 bg-slate-950/40 border border-white/5 rounded-xl p-5 flex flex-col justify-between relative overflow-hidden">
              <div className="absolute top-0 right-0 w-24 h-24 bg-indigo-500/5 rounded-full blur-2xl -z-10"></div>
              
              <div>
                <div className="flex items-center space-x-2 border-b border-white/5 pb-2 mb-3">
                  <span className="flex h-2 w-2 relative">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75"></span>
                    <span className="relative inline-flex rounded-full h-2 w-2 bg-indigo-500"></span>
                  </span>
                  <h4 className="text-xs font-bold text-gray-300 uppercase tracking-wider">Topology Inspector</h4>
                </div>

                {hoveredTopologyNode ? (
                  <div className="space-y-3.5 animate-fadeIn">
                    <div className="flex justify-between items-center">
                      <span className="text-sm font-bold text-indigo-400">{TOPOLOGY_INFO[hoveredTopologyNode].title}</span>
                      <span className="bg-indigo-950/60 border border-indigo-500/30 text-indigo-300 text-[8px] px-1.5 py-0.5 rounded font-mono uppercase tracking-widest">
                        {TOPOLOGY_INFO[hoveredTopologyNode].badge}
                      </span>
                    </div>

                    <div className="font-mono text-[10px] space-y-1 bg-slate-950/60 p-2 rounded-lg border border-white/5">
                      <div className="flex justify-between">
                        <span className="text-gray-500">ENGINE:</span>
                        <span className="text-gray-300">{TOPOLOGY_INFO[hoveredTopologyNode].tech}</span>
                      </div>
                      <div className="flex justify-between mt-1">
                        <span className="text-gray-500">PORT/PROTO:</span>
                        <span className="text-indigo-300 font-semibold">{TOPOLOGY_INFO[hoveredTopologyNode].protocol}</span>
                      </div>
                    </div>

                    <p className="text-gray-400 text-xs leading-relaxed">
                      {TOPOLOGY_INFO[hoveredTopologyNode].description}
                    </p>
                  </div>
                ) : (
                  <div className="text-center py-10 flex flex-col items-center justify-center space-y-3">
                    <span className="text-2xl opacity-40 animate-bounce">🔍</span>
                    <p className="text-gray-500 text-xs leading-relaxed font-mono">
                      Hover over any node or container in the diagram to inspect microservice details, proxy routes, and deployment states.
                    </p>
                  </div>
                )}
              </div>

              <div className="mt-6 pt-3 border-t border-white/5 text-[9px] text-gray-500 font-mono flex items-center justify-between">
                <span>DEPLOYMENT: live</span>
                <span>VM: azure-standard-b1s</span>
              </div>
            </div>

          </div>
        </section>
 
        {/* Featured Projects Section */}
        <section className="space-y-6">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-white/5 pb-3">
            <h3 className="text-2xl font-extrabold text-white tracking-wide">Featured Architecture & Code</h3>
            
            {/* Project Filter Controls */}
            <div className="flex flex-wrap gap-2">
              <button
                onClick={() => setSelectedTech(null)}
                className={`px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer ${!selectedTech ? 'bg-indigo-600 border-indigo-500 text-white shadow-[0_0_8px_#6366f1]' : 'bg-slate-800/60 border-white/5 text-gray-400 hover:text-white'}`}
              >
                All
              </button>
              {allTechTags.map(tag => (
                <button
                  key={tag}
                  onClick={() => setSelectedTech(tag === selectedTech ? null : tag)}
                  className={`px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer ${tag === selectedTech ? 'bg-indigo-600 border-indigo-500 text-white shadow-[0_0_8px_#6366f1]' : 'bg-slate-800/60 border-white/5 text-gray-400 hover:text-white hover:border-white/10'}`}
                >
                  {tag}
                </button>
              ))}
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 transition-all duration-500">
            {filteredProjects.map((project, index) => {
              const isDevOpsControlCenter = project.title === "DevOps Control Center";
              return (
                <div key={index} className={`bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl border transition-all duration-500 flex flex-col justify-between group hover:shadow-lg hover:shadow-indigo-950/5 ${selectedTech && project.tech.includes(selectedTech) ? 'border-indigo-500/50 shadow-[0_0_12px_rgba(99,102,241,0.05)]' : 'border-white/5 hover:border-indigo-500/30'}`}>
                  <div>
                    <h4 className="text-lg font-bold text-gray-200 mb-2 group-hover:text-indigo-400 transition-colors tracking-wide">{project.title}</h4>
                    <p className="text-gray-400 text-sm leading-relaxed mb-4">{project.description}</p>
                    
                    {isDevOpsControlCenter && (
                      <button 
                        onClick={() => setShowDevOpsCaseStudy(true)}
                        className="mb-6 px-4 py-2 bg-indigo-600/10 border border-indigo-500/30 text-indigo-400 hover:bg-indigo-600/20 hover:text-indigo-300 hover:border-indigo-500/60 rounded-xl text-xs font-bold transition-all duration-300 w-full cursor-pointer flex items-center justify-center space-x-1.5"
                      >
                        <span>⚙️</span>
                        <span>System Design & Demo</span>
                      </button>
                    )}
                  </div>
                  <div>
                    <div className="flex flex-wrap gap-1.5 mb-5">
                      {project.tech.map((tech, i) => (
                        <span 
                          key={i} 
                          onClick={() => setSelectedTech(tech === selectedTech ? null : tech)}
                          className={`border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 ${tech === selectedTech ? 'bg-indigo-600/20 border-indigo-500 text-indigo-300 shadow-[0_0_8px_rgba(99,102,241,0.2)]' : 'bg-slate-800/60 border-white/5 text-gray-300 hover:text-white hover:border-white/10'}`}
                        >
                          {tech}
                        </span>
                      ))}
                    </div>
                    <a 
                      href={project.link} 
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-indigo-400 text-xs font-bold hover:text-indigo-300 flex items-center transition-colors group/link"
                    >
                      View Repository <span className="ml-1 group-hover/link:translate-x-1 transition-transform">→</span>
                    </a>
                  </div>
                </div>
              );
            })}
          </div>
        </section>
 
      </main>

      {/* DevOps Control Center Case Study Modal */}
      {showDevOpsCaseStudy && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-md transition-opacity duration-300 overflow-y-auto">
          <div className="relative bg-[#0d1321] border border-white/10 rounded-2xl w-full max-w-3xl p-6 md:p-8 shadow-2xl overflow-hidden max-h-[90vh] overflow-y-auto custom-scrollbar">
            
            {/* Background glowing design elements */}
            <div className="absolute top-0 left-1/4 w-72 h-72 bg-indigo-600/10 rounded-full blur-3xl -z-10"></div>
            <div className="absolute bottom-0 right-1/4 w-72 h-72 bg-emerald-600/5 rounded-full blur-3xl -z-10"></div>

            {/* Header */}
            <div className="flex justify-between items-start border-b border-white/5 pb-4 mb-6">
              <div>
                <span className="text-[10px] font-bold text-indigo-400 uppercase tracking-widest">Case Study & Architecture</span>
                <h3 className="text-2xl font-extrabold text-white mt-1">DevOps Control Center</h3>
                <p className="text-xs text-gray-400 mt-1">A custom end-to-end telemetry and K8s orchestration dashboard.</p>
              </div>
              <button 
                onClick={() => setShowDevOpsCaseStudy(false)}
                className="text-gray-400 hover:text-white p-1 hover:bg-white/5 rounded-lg transition-colors cursor-pointer"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>

            {/* Content Body */}
            <div className="space-y-6">
              
              {/* Overview */}
              <div>
                <h4 className="text-xs font-bold uppercase text-gray-400 tracking-wider mb-2">Platform Overview</h4>
                <p className="text-gray-300 text-sm leading-relaxed">
                  This custom dashboard unifies server monitoring, remote terminal execution, Kubernetes deployment management, and CI/CD tracking into a single view. By proxying WebSocket traffic and stream channels securely, it allows remote administration from any browser interface.
                </p>
              </div>

              {/* Terminal Simulator Showcase */}
              <div>
                <h4 className="text-xs font-bold uppercase text-gray-400 tracking-wider mb-2">Simulated Interactive PTY Terminal</h4>
                <p className="text-gray-400 text-[11px] mb-3 leading-relaxed">
                  Below is a visual simulation of the live PTY console connection which streams raw shell sessions over secure WebSockets directly proxied from the Spring gateway to the Rust systems agent.
                </p>
                <TerminalSimulator active={showDevOpsCaseStudy} />
              </div>

              {/* System Architecture Diagram */}
              <div>
                <h4 className="text-xs font-bold uppercase text-gray-400 tracking-wider mb-1">Microservices Topology</h4>
                <p className="text-gray-400 text-[11px] mb-4 leading-relaxed">
                  The infrastructure operates inside the K3s namespace <code className="text-indigo-400 font-mono text-[10px] bg-indigo-950/40 px-1 py-0.5 rounded">devops</code> behind an Nginx reverse proxy.
                </p>

                {/* Architecture Visual Grid */}
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4 items-stretch justify-center my-4 py-4 px-3 border border-white/5 bg-white/2 rounded-xl">
                  {/* Client */}
                  <div className="flex flex-col items-center justify-between bg-slate-950/40 p-3 rounded-lg border border-white/5 text-center shadow-md">
                    <span className="text-xl">🌐</span>
                    <span className="text-[10px] font-bold text-gray-200 uppercase tracking-widest mt-1">Client Dashboard</span>
                    <span className="text-[9px] text-gray-500 mt-1">Vite + React UI Client</span>
                  </div>
                  {/* Spring Gateway */}
                  <div className="flex flex-col items-center justify-between bg-emerald-950/20 p-3 rounded-lg border border-emerald-500/20 text-center shadow-md">
                    <span className="text-xl">☕</span>
                    <span className="text-[10px] font-bold text-emerald-400 uppercase tracking-widest mt-1">Spring Gateway</span>
                    <span className="text-[9px] text-gray-400 mt-1">JWT Security Auth & WebSockets Proxy</span>
                  </div>
                  {/* Rust Agent */}
                  <div className="flex flex-col items-center justify-between bg-orange-950/20 p-3 rounded-lg border border-orange-500/20 text-center shadow-md">
                    <span className="text-xl">🦀</span>
                    <span className="text-[10px] font-bold text-orange-400 uppercase tracking-widest mt-1">Rust Agent</span>
                    <span className="text-[9px] text-gray-400 mt-1">kube-rs Client & Shell PTY Bridge</span>
                  </div>
                  {/* K3s API */}
                  <div className="flex flex-col items-center justify-between bg-blue-950/20 p-3 rounded-lg border border-blue-500/20 text-center shadow-md">
                    <span className="text-xl">☸️</span>
                    <span className="text-[10px] font-bold text-blue-400 uppercase tracking-widest mt-1">K3s Cluster API</span>
                    <span className="text-[9px] text-gray-400 mt-1">Pod Logs / Replicas & Status control</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Modal Footer Controls */}
            <div className="mt-8 pt-4 border-t border-white/5 flex flex-col sm:flex-row gap-3 justify-end items-center">
              <a 
                href="https://devops.mattdev0.tech" 
                target="_blank" 
                rel="noopener noreferrer" 
                className="w-full sm:w-auto px-5 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl text-xs text-center transition-colors shadow-lg shadow-indigo-900/20 cursor-pointer flex items-center justify-center space-x-1.5"
              >
                <span>🚀</span>
                <span>Launch Live Demo (Guest Mode)</span>
              </a>
              <a 
                href="https://github.com/mattDev0/devops-control-center" 
                target="_blank" 
                rel="noopener noreferrer" 
                className="w-full sm:w-auto px-5 py-2.5 bg-white/5 border border-white/10 hover:bg-white/10 hover:border-white/20 text-gray-300 hover:text-white font-semibold rounded-xl text-xs text-center transition-colors cursor-pointer"
              >
                View Repository Code
              </a>
              <button 
                onClick={() => setShowDevOpsCaseStudy(false)}
                className="w-full sm:w-auto px-5 py-2.5 bg-slate-800 hover:bg-slate-700 text-gray-300 hover:text-white font-semibold rounded-xl text-xs cursor-pointer"
              >
                Close Case Study
              </button>
            </div>
            
            <p className="text-[9px] text-gray-500 text-center mt-4">
              💡 Tip: On the live dashboard, you can bypass JWT login by clicking the "Guest Login" button to explore in read-only mode.
            </p>

          </div>
        </div>
      )}
    </div>
  );
}

export default App;
