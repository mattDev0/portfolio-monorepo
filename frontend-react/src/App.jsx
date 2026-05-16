import { useState, useEffect } from 'react';
import portfolioConfig from './config.json';

function App() {
  const [rustStatus, setRustStatus] = useState(null);
  const [javaStatus, setJavaStatus] = useState(null);
  const [spotifyData, setSpotifyData] = useState(null);

  // Fetch Rust Hardware Metrics
  useEffect(() => {
    fetch('http://localhost:8080/api/status')
      .then(response => response.json())
      .then(data => setRustStatus(data))
      .catch(error => console.error("Error fetching from Rust API:", error));
  }, []);

  // Fetch Java JVM Metrics
  useEffect(() => {
    fetch('http://localhost:8081/api/infrastructure/metrics')
      .then(response => response.json())
      .then(data => setJavaStatus(data))
      .catch(error => console.error("Error fetching from Java API:", error));
  }, []);

  // Fetch Live Spotify Session via Rust
  useEffect(() => {
    fetch('http://localhost:8080/api/spotify')
      .then(response => response.json())
      .then(data => setSpotifyData(data))
      .catch(error => console.error("Error fetching Spotify API:", error));
  }, []);

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col items-center py-12 px-4 sm:px-8">
      
      {/* Header Section */}
      <header className="text-center mb-12">
        <h1 className="text-5xl font-bold text-blue-400 mb-2 tracking-tight">{portfolioConfig.name}</h1>
        <h2 className="text-2xl font-semibold text-gray-300 mb-2">{portfolioConfig.title}</h2>
        <div className="flex items-center justify-center space-x-2 text-gray-400 mb-6">
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
          <span>{portfolioConfig.location}</span>
        </div>
        <p className="max-w-2xl mx-auto text-gray-400 leading-relaxed italic">
          "{portfolioConfig.tagline}"
        </p>
      </header>
      
      <main className="w-full max-w-5xl space-y-12">
        
        {/* Profile & Experience Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-gray-700 md:col-span-2 flex flex-col justify-between">
            <div>
              <h3 className="text-xl font-bold text-white mb-4 border-b border-gray-600 pb-2">About the Developer</h3>
              <p className="text-gray-300 leading-relaxed text-sm">
                {portfolioConfig.about}
              </p>
            </div>
            <div className="mt-6 pt-6 border-t border-gray-700">
               <h4 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2">Education</h4>
               <div className="flex justify-between items-center">
                 <span className="text-gray-200 font-medium">{portfolioConfig.education.degree}</span>
                 <span className="text-blue-400 text-sm">{portfolioConfig.education.year}</span>
               </div>
               <div className="text-gray-500 text-sm">{portfolioConfig.education.institution}</div>
            </div>
          </section>

          <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-gray-700">
            <h3 className="text-xl font-bold text-white mb-4 border-b border-gray-600 pb-2">Recent Roles</h3>
            <div className="space-y-6">
              {portfolioConfig.experience.map((exp, index) => (
                <div key={index} className="relative pl-4 border-l-2 border-blue-500/50">
                  <div className="absolute w-2 h-2 bg-blue-500 rounded-full -left-[5px] top-1.5"></div>
                  <h4 className="text-gray-200 font-semibold text-sm">{exp.role}</h4>
                  <p className="text-blue-400 text-xs mb-1">{exp.company}</p>
                  <p className="text-gray-400 text-xs leading-relaxed">{exp.description}</p>
                </div>
              ))}
            </div>
          </section>
        </div>

        {/* Featured Projects Section */}
        <section className="space-y-6">
          <h3 className="text-2xl font-bold text-white border-b border-gray-700 pb-2">Featured Architecture & Code</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {portfolioConfig.projects.map((project, index) => (
              <div key={index} className="bg-gray-800 p-6 rounded-xl border border-gray-700 hover:border-blue-500/50 transition-colors duration-300 flex flex-col justify-between group">
                <div>
                  <h4 className="text-lg font-bold text-gray-200 mb-2 group-hover:text-blue-400 transition-colors">{project.title}</h4>
                  <p className="text-gray-400 text-sm leading-relaxed mb-4">{project.description}</p>
                </div>
                <div>
                  <div className="flex flex-wrap gap-2 mb-4">
                    {project.tech.map((tech, i) => (
                      <span key={i} className="bg-gray-700 text-gray-300 text-xs px-2 py-1 rounded">{tech}</span>
                    ))}
                  </div>
                  <a href={project.link} className="text-blue-400 text-sm font-semibold hover:text-blue-300 flex items-center">
                    View Repository <span className="ml-1">→</span>
                  </a>
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Dynamic Microservices & Live Data Grid */}
        <section className="space-y-6">
          <h3 className="text-2xl font-bold text-white border-b border-gray-700 pb-2">Live Systems & Telemetry</h3>
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
            
            {/* Rust Engine */}
            <section className="bg-gray-800 p-6 rounded-xl shadow-lg border border-orange-500/30 relative overflow-hidden">
              <div className="absolute top-0 right-0 p-4">
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-orange-500"></span>
                </span>
              </div>
              <h3 className="text-lg font-bold text-orange-400 mb-4 border-b border-gray-600 pb-2">Rust Engine</h3>
              {rustStatus ? (
                <div className="font-mono text-xs space-y-3 mt-2">
                  <div className="flex justify-between border-b border-gray-700 pb-1">
                    <span className="text-gray-500 uppercase">OS</span>
                    <span className="text-gray-200">{rustStatus.os_info}</span>
                  </div>
                  <div className="flex justify-between border-b border-gray-700 pb-1">
                    <span className="text-gray-500 uppercase">Threads</span>
                    <span className="text-blue-400">{rustStatus.cpu_cores}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500 uppercase">Memory</span>
                    <span className="text-purple-400">{rustStatus.memory_usage}</span>
                  </div>
                </div>
              ) : (
                <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Handshake pending...</p>
              )}
            </section>

            {/* Java Infrastructure */}
            <section className="bg-gray-800 p-6 rounded-xl shadow-lg border border-green-500/30 relative overflow-hidden">
              <div className="absolute top-0 right-0 p-4">
                <span className="flex h-3 w-3 relative">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
                </span>
              </div>
              <h3 className="text-lg font-bold text-green-400 mb-4 border-b border-gray-600 pb-2">Java Infrastructure</h3>
              {javaStatus ? (
                <div className="font-mono text-xs space-y-3 mt-2">
                  <div className="flex justify-between border-b border-gray-700 pb-1">
                    <span className="text-gray-500 uppercase">Version</span>
                    <span className="text-gray-200">{javaStatus.engine}</span>
                  </div>
                  <div className="flex justify-between border-b border-gray-700 pb-1">
                    <span className="text-gray-500 uppercase">Uptime</span>
                    <span className="text-blue-400">{javaStatus.uptime} ({javaStatus.active_threads})</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-500 uppercase">JVM Mem</span>
                    <span className="text-cyan-400">{javaStatus.jvm_memory}</span>
                  </div>
                </div>
              ) : (
                <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Handshake pending...</p>
              )}
            </section>

            {/* Live Spotify Session via Rust */}
            <section className="bg-gray-800 p-6 rounded-xl shadow-lg border border-[#1DB954]/30 relative overflow-hidden flex flex-col justify-center">
              {spotifyData?.is_playing && (
                <div className="absolute top-0 right-0 p-4">
                  <div className="flex space-x-1 items-end h-3">
                    <div className="w-1 bg-[#1DB954] animate-[bounce_1s_infinite_0ms] h-full"></div>
                    <div className="w-1 bg-[#1DB954] animate-[bounce_1s_infinite_200ms] h-2/3"></div>
                    <div className="w-1 bg-[#1DB954] animate-[bounce_1s_infinite_400ms] h-full"></div>
                  </div>
                </div>
              )}
              
              <h3 className="text-lg font-bold text-[#1DB954] mb-4 border-b border-gray-600 pb-2 flex items-center">
                <svg className="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z"/></svg>
                Shared Session
              </h3>
              
              {spotifyData ? (
                <div className="flex items-center space-x-4 mt-2">
                  <div className="w-16 h-16 bg-gray-700 rounded-md flex-shrink-0 shadow-md overflow-hidden relative">
                    {spotifyData.album_art ? (
                      <img src={spotifyData.album_art} alt="Album Art" className="w-full h-full object-cover" />
                    ) : (
                      <div className="absolute inset-0 bg-gradient-to-tr from-[#1DB954] to-gray-900 opacity-50"></div>
                    )}
                  </div>
                  <div className="overflow-hidden">
                    <p className="text-gray-200 font-bold truncate">{spotifyData.title}</p>
                    <p className="text-gray-400 text-sm truncate">{spotifyData.artist}</p>
                    <p className={`text-xs mt-1 font-mono tracking-wider uppercase ${spotifyData.is_playing ? 'text-[#1DB954]' : 'text-gray-500'}`}>
                      {spotifyData.is_playing ? 'Live Sync Active' : 'Offline'}
                    </p>
                  </div>
                </div>
              ) : (
                <p className="text-gray-500 animate-pulse font-mono text-xs mt-2">Connecting to Spotify Web API...</p>
              )}
            </section>

          </div>
        </section>

      </main>
    </div>
  );
}

export default App;