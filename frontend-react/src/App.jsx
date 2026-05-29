import { useState, useEffect } from 'react';
import portfolioConfig from './config.json';
import GitHubActivity from './GitHubActivity';
import { apiUrl } from './api';

function Sparkline({ data, color }) {
  if (!data || data.length < 2) {
    return (
      <div className="h-4 flex items-center justify-center text-[8px] text-gray-600 font-mono tracking-wider opacity-50">
        syncing...
      </div>
    );
  }

  const height = 18;
  const width = 80;
  const padding = 1;

  const minVal = 0;
  const maxVal = 100;

  const points = data.map((val, index) => {
    const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
    const y = padding + (1 - (val - minVal) / (maxVal - minVal || 1)) * (height - 2 * padding);
    return `${x},${y}`;
  }).join(' ');

  return (
    <svg className="w-[80px] h-[18px] opacity-85" viewBox={`0 0 ${width} ${height}`}>
      <polyline
        fill="none"
        stroke={color}
        strokeWidth="1.2"
        strokeLinecap="round"
        strokeLinejoin="round"
        points={points}
      />
    </svg>
  );
}

function TerminalSimulator({ active }) {
  const [lines, setLines] = useState([]);
  
  const terminalSequence = [
    { text: "guest@mattdev0.tech:~$ initialize_telemetry", delay: 200 },
    { text: "[INFO] Establishing WebSocket handshake with rust-agent...", delay: 600 },
    { text: "[INFO] Client handshake accepted. Handshaking Spring orchestrator...", delay: 500 },
    { text: "[INFO] Session validated via JWT authority claims (ROLE_GUEST).", delay: 400 },
    { text: "[SUCCESS] Bidirectional PTY bridge established.", delay: 300 },
    { text: "guest@mattdev0.tech:~$ kubectl get pods -n devops", delay: 500 },
    { text: "NAME                                READY   STATUS    RESTARTS   AGE", delay: 300 },
    { text: "devops-frontend-6b4d5b6c-8x2p       1/1     Running   0          45d", delay: 100 },
    { text: "devops-orchestrator-9c4d5b6-7y1q    1/1     Running   2          45d", delay: 100 },
    { text: "devops-agent-8b3d4a5-9z0p           1/1     Running   0          45d", delay: 100 },
    { text: "guest@mattdev0.tech:~$ sysinfo --telemetry", delay: 600 },
    { text: "[OS] Linux 6.1.0-azure-amd64 | [Telemetry Mode] Active SSE Streams", delay: 400 },
    { text: "[Cluster metrics] Prometheus Node Exporter DaemonSet operational.", delay: 200 },
    { text: "guest@mattdev0.tech:~$ █", delay: 800 }
  ];

  useEffect(() => {
    if (!active) {
      setLines([]);
      return;
    }

    let isMounted = true;
    let currentLineIndex = 0;
    
    const runSequence = async () => {
      for (const step of terminalSequence) {
        if (!isMounted) break;
        await new Promise(resolve => setTimeout(resolve, step.delay));
        if (!isMounted) break;
        setLines(prev => [...prev, step.text]);
      }
    };

    runSequence();

    return () => {
      isMounted = false;
    };
  }, [active]);

  return (
    <div className="bg-[#050811] border border-white/5 rounded-xl p-4 font-mono text-[11px] text-indigo-300 leading-relaxed shadow-inner h-60 overflow-y-auto custom-scrollbar select-none">
      {lines.map((line, idx) => {
        if (line.startsWith("guest@")) {
          return (
            <div key={idx} className="text-gray-400 mt-2">
              <span className="text-emerald-400">guest@mattdev0.tech</span>:<span className="text-blue-400">~</span>$ {line.substring(23)}
            </div>
          );
        }
        if (line.startsWith("[SUCCESS]")) {
          return <div key={idx} className="text-emerald-400 font-semibold">{line}</div>;
        }
        if (line.startsWith("[INFO]")) {
          return <div key={idx} className="text-gray-500">{line}</div>;
        }
        return <div key={idx} className="text-gray-200">{line}</div>;
      })}
    </div>
  );
}

const TOPOLOGY_INFO = {
  client: {
    title: "Client Browser",
    tech: "React 19 + Vite + Tailwind CSS",
    protocol: "HTTPS (Port 443)",
    description: "The interactive UI dashboard running in the visitor's browser. It polls the microservice APIs periodically (Telemetry every 5s, JVM every 10s, Spotify every 10s) and updates the layout reactively.",
    badge: "Frontend"
  },
  nginx: {
    title: "Nginx Reverse Proxy",
    tech: "Nginx Gateway Config",
    protocol: "HTTPS -> NodePorts",
    description: "Operates as the entry point on the Azure VM. It terminates SSL (Let's Encrypt), resolves CORS by hosting services on the same domain, and routes paths: / to Port 30000, /api/status & /api/spotify to Port 30080, and /api/github to Port 30081.",
    badge: "Gateway"
  },
  k8s: {
    title: "K3s Kubernetes Namespace",
    tech: "K3s Cluster (Orchestration)",
    protocol: "K8s Service DNS",
    description: "A lightweight, secure Kubernetes namespace ('portfolio') hosting isolated container pods. Manages scaling, self-healing, rolling updates, and enforces strict resource limits (limits: memory: 32Mi, cpu: 100m).",
    badge: "Infrastructure"
  },
  frontend: {
    title: "frontend-react Pod",
    tech: "Docker + Nginx Server",
    protocol: "HTTP (Container Port 80)",
    description: "Containerized deployment serving the static built React application bundle. Managed via rolling zero-downtime restarts in Kubernetes (K3s NodePort 30000).",
    badge: "Deployment Pod"
  },
  rust: {
    title: "backend-rust Pod",
    tech: "Rust + Axum Engine",
    protocol: "HTTP (Container Port 8080)",
    description: "High-performance Axum web server compiled to native machine code. Refreshes host CPU/Memory telemetry in a thread-safe background task and acts as the Spotify authentication token manager.",
    badge: "Deployment Pod"
  },
  java: {
    title: "backend-java Pod",
    tech: "Java 21 + Spring Boot 4",
    protocol: "HTTP (Container Port 8081)",
    description: "Enterprise-grade Spring Boot microservice. Manages caching abstractions using Spring Cache (@Cacheable) to bypass public rate boundaries, pulling recent commit payloads from the GitHub API.",
    badge: "Deployment Pod"
  },
  spotify: {
    title: "Spotify Web API",
    tech: "OAuth 2.0 REST Web Services",
    protocol: "External API Calls",
    description: "Third-party audio streaming REST API. Authenticated securely via OAuth 2.0 client-credentials and token refresh flows executed from the Rust backend to fetch current/recent tracks.",
    badge: "External Service"
  },
  github: {
    title: "GitHub REST API",
    tech: "GitHub Public REST API",
    protocol: "External API Calls",
    description: "Third-party platform API queried by the Java backend to fetch live repository events. Spring Cache limits API calls to prevent IP rate-limiting blocks.",
    badge: "External Service"
  }
};

function App() {
  const [rustStatus, setRustStatus] = useState(null);
  const [javaStatus, setJavaStatus] = useState(null);
  const [spotifyData, setSpotifyData] = useState(null);
  const [localProgressMs, setLocalProgressMs] = useState(0);
  const [selectedTech, setSelectedTech] = useState(null);
  const [telemetryHistory, setTelemetryHistory] = useState([]);
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

  const renderNode = (id, icon, title, tech) => {
    const isHovered = hoveredTopologyNode === id;
    
    let accentBorder = "border-white/5";
    let accentBg = "bg-slate-950/20 hover:border-indigo-500/20";
    let accentText = "text-gray-300";
    
    if (isHovered) {
      if (id === 'rust') {
        accentBorder = "border-orange-500/40 shadow-[0_0_12px_rgba(249,115,22,0.15)] animate-pulse";
        accentBg = "bg-orange-500/5";
        accentText = "text-orange-400";
      } else if (id === 'java') {
        accentBorder = "border-emerald-500/40 shadow-[0_0_12px_rgba(16,185,129,0.15)] animate-pulse";
        accentBg = "bg-emerald-500/5";
        accentText = "text-emerald-400";
      } else if (id === 'frontend' || id === 'client') {
        accentBorder = "border-blue-500/40 shadow-[0_0_12px_rgba(59,130,246,0.15)] animate-pulse";
        accentBg = "bg-blue-500/5";
        accentText = "text-blue-400";
      } else if (id === 'nginx') {
        accentBorder = "border-indigo-500/40 shadow-[0_0_12px_rgba(99,102,241,0.15)] animate-pulse";
        accentBg = "bg-indigo-500/5";
        accentText = "text-indigo-400";
      } else if (id === 'k8s') {
        accentBorder = "border-blue-400/50 shadow-[0_0_12px_rgba(96,165,250,0.2)]";
        accentBg = "bg-blue-400/5";
        accentText = "text-blue-400";
      } else {
        accentBorder = "border-indigo-500/40 shadow-[0_0_12px_rgba(99,102,241,0.15)]";
        accentBg = "bg-indigo-500/5";
        accentText = "text-indigo-400";
      }
    }

    return (
      <div
        onMouseEnter={() => setHoveredTopologyNode(id)}
        onMouseLeave={() => setHoveredTopologyNode(null)}
        className={`p-3 md:p-3.5 rounded-xl border flex flex-col items-center justify-center text-center transition-all duration-300 cursor-help select-none ${accentBorder} ${accentBg} ${isHovered ? 'scale-[1.02]' : ''}`}
      >
        <span className="text-lg md:text-xl mb-0.5">{icon}</span>
        <span className={`text-[9px] md:text-[10px] font-bold uppercase tracking-wider ${accentText}`}>{title}</span>
        <span className="text-[8px] md:text-[9px] text-gray-500 font-mono mt-0.5">{tech}</span>
      </div>
    );
  };

  // Extract all unique technology tags from projects
  const allTechTags = Array.from(
    new Set(portfolioConfig.projects.flatMap(p => p.tech))
  );

  const filteredProjects = selectedTech
    ? portfolioConfig.projects.filter(p => p.tech.includes(selectedTech))
    : portfolioConfig.projects;

  // Fetch Rust Hardware Metrics (with 5s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchRust = () => {
      fetch(apiUrl('rust', '/api/status'))
        .then(response => response.json())
        .then(data => setRustStatus(data))
        .catch(error => console.error("Error fetching from Rust API:", error));
    };

    fetchRust();
    const interval = setInterval(fetchRust, 5000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Fetch Rust Telemetry History on mount (and on regaining visibility if history was blank)
  useEffect(() => {
    if (!isVisible) return;
    fetch(apiUrl('rust', '/api/status/history'))
      .then(response => response.json())
      .then(data => setTelemetryHistory(data))
      .catch(error => console.error("Error fetching telemetry history:", error));
  }, [isVisible]);

  // Sync polled metrics with history buffer
  useEffect(() => {
    if (!rustStatus || !isVisible) return;

    const cpu = getCpuPercentage(rustStatus.cpu_usage);

    // Parse memory percentage from "used MB / total MB"
    let memory = 0;
    if (rustStatus.memory_usage) {
      const parts = rustStatus.memory_usage.split('/');
      if (parts.length === 2) {
        const used = parseFloat(parts[0].replace('MB', '').trim());
        const total = parseFloat(parts[1].replace('MB', '').trim());
        if (total > 0) {
          memory = (used / total) * 100;
        }
      }
    }

    setTelemetryHistory(prev => {
      // Check if this new data point is already the latest point in history to prevent duplicate entries
      const lastPoint = prev[prev.length - 1];
      if (lastPoint && lastPoint.cpu === cpu && lastPoint.memory === memory) {
        return prev;
      }
      const newHistory = [...prev, { cpu, memory }];
      if (newHistory.length > 20) {
        newHistory.shift();
      }
      return newHistory;
    });
  }, [rustStatus, isVisible]);

  // Fetch Java JVM Metrics (with 10s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchJava = () => {
      fetch(apiUrl('java', '/api/infrastructure/metrics'))
        .then(response => response.json())
        .then(data => setJavaStatus(data))
        .catch(error => console.error("Error fetching from Java API:", error));
    };

    fetchJava();
    const interval = setInterval(fetchJava, 10000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Fetch Live Spotify Session via Rust (with 10s polling, only when tab is visible)
  useEffect(() => {
    if (!isVisible) return;
    const fetchSpotify = () => {
      fetch(apiUrl('rust', '/api/spotify'))
        .then(response => response.json())
        .then(data => setSpotifyData(data))
        .catch(error => console.error("Error fetching Spotify API:", error));
    };

    fetchSpotify();
    const interval = setInterval(fetchSpotify, 10000);
    return () => clearInterval(interval);
  }, [isVisible]);

  // Sync local progress when new data is fetched
  useEffect(() => {
    if (spotifyData) {
      setLocalProgressMs(spotifyData.progress_ms || 0);
    }
  }, [spotifyData]);

  // Tick the progress bar locally every second if a song is playing (only when tab is visible)
  useEffect(() => {
    if (!spotifyData || !spotifyData.is_playing || !isVisible) return;

    const interval = setInterval(() => {
      setLocalProgressMs(prev => {
        const next = prev + 1000;
        return next > spotifyData.duration_ms ? spotifyData.duration_ms : next;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [spotifyData, isVisible]);

  // Helper to format milliseconds to M:SS
  const formatTime = (ms) => {
    if (!ms) return "0:00";
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
  };

  const getCpuPercentage = (cpuUsageStr) => {
    if (!cpuUsageStr) return 0;
    const parsed = parseFloat(cpuUsageStr.replace('%', ''));
    return isNaN(parsed) ? 0 : parsed;
  };

  const progressPercent = spotifyData?.duration_ms
    ? (localProgressMs / spotifyData.duration_ms) * 100
    : 0;

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
          
          {/* --- LEFT COLUMN: About & GitHub Activity (Spans 2 of 3 columns) --- */}
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
 
            {/* Live GitHub Activity */}
            <GitHubActivity />
            
          </div>
          {/* --- END LEFT COLUMN --- */}
 
          {/* --- RIGHT COLUMN: Experience & Telemetry (Spans 1 of 3 columns) --- */}
          <div className="lg:col-span-1 flex flex-col gap-8">
            
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

            {/* Live Telemetry Stack */}
            <div className="flex flex-col gap-6">
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
                          <Sparkline data={telemetryHistory.map(h => h.cpu)} color="#f97316" />
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
                        <Sparkline data={telemetryHistory.map(h => h.memory)} color="#f59e0b" />
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
   
              {/* Live Spotify Session via Rust */}
              {spotifyData?.track_url ? (
                <a 
                  href={spotifyData.track_url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-emerald-500/10 hover:border-emerald-500/30 relative overflow-hidden transition-all duration-300 group hover:shadow-lg hover:shadow-emerald-950/5 flex flex-col justify-center cursor-pointer"
                >
                  {renderSpotifyInner(spotifyData, progressPercent, localProgressMs, formatTime)}
                </a>
              ) : (
                <section className="bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl shadow-xl border border-white/5 relative overflow-hidden transition-all duration-300 flex flex-col justify-center">
                  {renderSpotifyInner(spotifyData, progressPercent, localProgressMs, formatTime)}
                </section>
              )}
              
            </div>
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
                  {renderNode('client', '🌐', 'Client Browser', 'React 19 + Vite')}
                </div>

                {/* Arrow Client -> Nginx */}
                <div className="flex items-center justify-center text-gray-700 font-mono text-xs py-1 md:py-0">
                  <span className="md:hidden">⬇️</span>
                  <span className="hidden md:inline text-indigo-400/40">── HTTPS ──▶</span>
                </div>

                {/* 2. Nginx Card */}
                <div className="flex-1 w-full flex flex-col justify-center">
                  {renderNode('nginx', '🛡️', 'Nginx Proxy', 'Port 443 SSL')}
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
                    {renderNode('frontend', '⚛️', 'Frontend Pod', 'NodePort 30000')}
                    {/* Pod 2: Rust API */}
                    {renderNode('rust', '🦀', 'Rust Pod', 'NodePort 30080')}
                    {/* Pod 3: Java API */}
                    {renderNode('java', '☕', 'Java Pod', 'NodePort 30081')}
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
                    {renderNode('spotify', '🎵', 'Spotify API', 'v1 Player Web Service')}
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
                    {renderNode('github', '🐙', 'GitHub API', 'v3 Public REST')}
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
                href="https://mattdev0.tech/devops/" 
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

// Separate helper function to render Spotify internals to keep code clean and readable
function renderSpotifyInner(spotifyData, progressPercent, localProgressMs, formatTime) {
  const isPlaying = spotifyData?.is_playing;
  const isRecentlyPlayed = spotifyData?.is_recently_played;
  return (
    <>
      <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
        <span className="flex h-3 w-3 relative">
          <span className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 ${isPlaying ? 'bg-emerald-400' : isRecentlyPlayed ? 'bg-amber-400' : 'bg-gray-500'}`}></span>
          {isPlaying && <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400/40 opacity-40 delay-300"></span>}
          <span className={`relative inline-flex rounded-full h-3 w-3 ${isPlaying ? 'bg-emerald-500' : isRecentlyPlayed ? 'bg-amber-500' : 'bg-gray-500'} ${isPlaying ? 'shadow-[0_0_8px_#10b981]' : isRecentlyPlayed ? 'shadow-[0_0_8px_#f59e0b]' : ''}`}></span>
        </span>
        {/* Custom Tooltip */}
        <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-slate-950/95 border border-emerald-500/20 p-2 text-center text-[10px] text-emerald-400 font-mono shadow-xl z-20">
          {isPlaying ? 'Live playback' : isRecentlyPlayed ? 'Playback idle' : 'Offline'}
        </div>
      </div>
      
      <h3 className="text-lg font-bold text-emerald-400 mb-0 tracking-wide flex items-center">
        <svg className="w-5 h-5 mr-2 text-emerald-400 group-hover:animate-spin" style={{ animationDuration: isPlaying ? '6s' : '0s' }} fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z"/></svg>
        Spotify Session
      </h3>
      <p className="text-[10px] text-gray-500 font-medium mb-5">Web API sync via Rust token auth</p>
      
      {spotifyData ? (
        <div>
          <div className="flex items-center space-x-4 mt-2">
            <div className={`w-16 h-16 bg-slate-800 rounded-lg flex-shrink-0 shadow-md overflow-hidden relative border border-white/5 transition-transform duration-500 ${isPlaying ? 'group-hover:rotate-12 group-hover:scale-105' : ''}`}>
              {spotifyData.album_art ? (
                <img src={spotifyData.album_art} alt="Album Art" className={`w-full h-full object-cover transition-all duration-500 ${isPlaying ? '' : 'grayscale opacity-60'}`} />
              ) : (
                <div className="absolute inset-0 bg-gradient-to-tr from-emerald-500 to-slate-900 opacity-50"></div>
              )}
            </div>
            <div className="overflow-hidden flex-grow">
              <p className="text-gray-100 font-bold truncate group-hover:text-emerald-400 transition-colors">{spotifyData.title}</p>
              <p className="text-gray-400 text-sm truncate">{spotifyData.artist}</p>
              <p className={`text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold ${isPlaying ? 'text-emerald-400' : isRecentlyPlayed ? 'text-amber-500/80' : 'text-gray-500'}`}>
                {isPlaying ? 'Now Playing' : isRecentlyPlayed ? 'Recently Played' : 'Offline'}
              </p>
            </div>
          </div>
          
          {isPlaying && spotifyData.duration_ms > 0 && (
            <div className="mt-5">
              <div className="w-full h-1 bg-slate-950 rounded-full overflow-hidden">
                <div 
                  className="h-full bg-emerald-500 transition-all duration-1000 ease-linear shadow-[0_0_8px_#10b981]"
                  style={{ width: `${progressPercent}%` }}
                ></div>
              </div>
              <div className="flex justify-between text-[9px] text-gray-500 mt-1.5 font-mono">
                <span>{formatTime(localProgressMs)}</span>
                <span>{formatTime(spotifyData.duration_ms)}</span>
              </div>
            </div>
          )}
        </div>
      ) : (
        <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Connecting to Spotify Web API...</p>
      )}
    </>
  );
}

export default App;
