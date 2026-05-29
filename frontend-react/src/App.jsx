import { useState, useEffect } from 'react';
import portfolioConfig from './config.json';
import GitHubActivity from './GitHubActivity';
import { apiUrl } from './api';

function App() {
  const [rustStatus, setRustStatus] = useState(null);
  const [javaStatus, setJavaStatus] = useState(null);
  const [spotifyData, setSpotifyData] = useState(null);
  const [localProgressMs, setLocalProgressMs] = useState(0);
  const [selectedTech, setSelectedTech] = useState(null);

  // Extract all unique technology tags from projects
  const allTechTags = Array.from(
    new Set(portfolioConfig.projects.flatMap(p => p.tech))
  );

  const filteredProjects = selectedTech
    ? portfolioConfig.projects.filter(p => p.tech.includes(selectedTech))
    : portfolioConfig.projects;

  // Fetch Rust Hardware Metrics (with 5s polling)
  useEffect(() => {
    const fetchRust = () => {
      fetch(apiUrl('rust', '/api/status'))
        .then(response => response.json())
        .then(data => setRustStatus(data))
        .catch(error => console.error("Error fetching from Rust API:", error));
    };

    fetchRust();
    const interval = setInterval(fetchRust, 5000);
    return () => clearInterval(interval);
  }, []);

  // Fetch Java JVM Metrics (with 10s polling)
  useEffect(() => {
    const fetchJava = () => {
      fetch(apiUrl('java', '/api/infrastructure/metrics'))
        .then(response => response.json())
        .then(data => setJavaStatus(data))
        .catch(error => console.error("Error fetching from Java API:", error));
    };

    fetchJava();
    const interval = setInterval(fetchJava, 10000);
    return () => clearInterval(interval);
  }, []);

  // Fetch Live Spotify Session via Rust (with 10s polling)
  useEffect(() => {
    const fetchSpotify = () => {
      fetch(apiUrl('rust', '/api/spotify'))
        .then(response => response.json())
        .then(data => setSpotifyData(data))
        .catch(error => console.error("Error fetching Spotify API:", error));
    };

    fetchSpotify();
    const interval = setInterval(fetchSpotify, 10000);
    return () => clearInterval(interval);
  }, []);

  // Sync local progress when new data is fetched
  useEffect(() => {
    if (spotifyData) {
      setLocalProgressMs(spotifyData.progress_ms || 0);
    }
  }, [spotifyData]);

  // Tick the progress bar locally every second if a song is playing
  useEffect(() => {
    if (!spotifyData || !spotifyData.is_playing) return;

    const interval = setInterval(() => {
      setLocalProgressMs(prev => {
        const next = prev + 1000;
        return next > spotifyData.duration_ms ? spotifyData.duration_ms : next;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [spotifyData]);

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
                      <div className="flex justify-between mb-1.5">
                        <span className="text-gray-500 uppercase text-[10px]">CPU Utilization</span>
                        <span className="text-orange-400 font-semibold">{rustStatus.cpu_usage || "0%"}</span>
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
                    <div className="flex justify-between">
                      <span className="text-gray-500 uppercase text-[10px]">Memory</span>
                      <span className="text-amber-500 font-semibold">{rustStatus.memory_usage}</span>
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
            {filteredProjects.map((project, index) => (
              <div key={index} className={`bg-slate-900/30 backdrop-blur-md p-6 rounded-2xl border transition-all duration-500 flex flex-col justify-between group hover:shadow-lg hover:shadow-indigo-950/5 ${selectedTech && project.tech.includes(selectedTech) ? 'border-indigo-500/50 shadow-[0_0_12px_rgba(99,102,241,0.05)]' : 'border-white/5 hover:border-indigo-500/30'}`}>
                <div>
                  <h4 className="text-lg font-bold text-gray-200 mb-2 group-hover:text-indigo-400 transition-colors tracking-wide">{project.title}</h4>
                  <p className="text-gray-400 text-sm leading-relaxed mb-6">{project.description}</p>
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
            ))}
          </div>
        </section>
 
 
      </main>
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
