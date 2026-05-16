import { useState, useEffect } from 'react';

function App() {
  const [rustStatus, setRustStatus] = useState(null);
  const [javaStatus, setJavaStatus] = useState(null);

  // Fetch Rust Microservice Data
  useEffect(() => {
    fetch('http://localhost:8080/api/status')
      .then(response => response.json())
      .then(data => setRustStatus(data))
      .catch(error => console.error("Error fetching from Rust API:", error));
  }, []);

  // Fetch Java Enterprise Data
  useEffect(() => {
    fetch('http://localhost:8081/api/infrastructure/metrics')
      .then(response => response.json())
      .then(data => setJavaStatus(data))
      .catch(error => console.error("Error fetching from Java API:", error));
  }, []);

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col items-center py-12 px-4 sm:px-8">
      
      <header className="text-center mb-12">
        <h1 className="text-5xl font-bold text-blue-400 mb-4 tracking-tight">Odunayo Abraham</h1>
        <h2 className="text-2xl font-semibold text-gray-300 mb-6">DevOps Engineer & Software Developer</h2>
        <p className="max-w-2xl mx-auto text-gray-400 leading-relaxed">
          Bridging low-level systems, high-performance backends, and modern web architecture.
        </p>
      </header>
      
      <main className="w-full max-w-5xl space-y-8">
        
        {/* Static Skills Section */}
        <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-gray-700">
          <h3 className="text-xl font-bold text-white mb-4 border-b border-gray-600 pb-2">Core Technologies</h3>
          <ul className="grid grid-cols-1 sm:grid-cols-2 gap-4 text-gray-300">
            <li className="flex items-center space-x-2">
              <span className="text-blue-500">▹</span>
              <span>Java & Spring Boot</span>
            </li>
            <li className="flex items-center space-x-2">
              <span className="text-orange-500">▹</span>
              <span>Rust & WebAssembly</span>
            </li>
            <li className="flex items-center space-x-2">
              <span className="text-yellow-500">▹</span>
              <span>Linux Systems Administration</span>
            </li>
            <li className="flex items-center space-x-2">
              <span className="text-green-500">▹</span>
              <span>Android OS Development</span>
            </li>
          </ul>
        </section>

        {/* Dynamic Microservices Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          
          {/* Rust API Section */}
          <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-orange-500/30 relative overflow-hidden">
            <div className="absolute top-0 right-0 p-4">
              <span className="flex h-3 w-3 relative">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-3 w-3 bg-orange-500"></span>
              </span>
            </div>
            <h3 className="text-xl font-bold text-orange-400 mb-4 border-b border-gray-600 pb-2">Rust Core Engine</h3>
            
            {rustStatus ? (
              <div className="font-mono text-sm space-y-3 mt-4">
                <div className="flex flex-col border-b border-gray-700 pb-2">
                  <span className="text-gray-500 text-xs uppercase tracking-wider mb-1">Host OS</span>
                  <span className="text-gray-200">{rustStatus.os_info}</span>
                </div>
                <div className="flex flex-col border-b border-gray-700 pb-2">
                  <span className="text-gray-500 text-xs uppercase tracking-wider mb-1">Hardware Threads</span>
                  <span className="text-blue-400">{rustStatus.cpu_cores}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500 text-xs uppercase tracking-wider mb-1">Live Memory Usage</span>
                  <span className="text-purple-400">{rustStatus.memory_usage}</span>
                </div>
              </div>
            ) : (
              <p className="text-gray-500 animate-pulse font-mono mt-4">Handshake pending on :8080...</p>
            )}
          </section>

          {/* Java API Section */}
          <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-green-500/30 relative overflow-hidden">
            <div className="absolute top-0 right-0 p-4">
              <span className="flex h-3 w-3 relative">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
              </span>
            </div>
            <h3 className="text-xl font-bold text-green-400 mb-4 border-b border-gray-600 pb-2">Java Infrastructure</h3>
            
            {javaStatus ? (
              <div className="font-mono text-sm space-y-3 mt-4">
                <div className="flex flex-col border-b border-gray-700 pb-2">
                  <span className="text-gray-500 text-xs uppercase tracking-wider mb-1">Engine Version</span>
                  <span className="text-gray-200">{javaStatus.engine}</span>
                </div>
                <div className="flex flex-col border-b border-gray-700 pb-2">
                  <span className="text-gray-500 text-xs uppercase tracking-wider mb-1">Server Uptime & Threads</span>
                  <span className="text-blue-400">Up {javaStatus.uptime} — {javaStatus.active_threads}</span>
                </div>
                <div className="flex flex-col">
                  <span className="text-gray-500 text-xs uppercase tracking-wider mb-1">Live JVM Memory Allocation</span>
                  <span className="text-cyan-400">{javaStatus.jvm_memory}</span>
                </div>
              </div>
            ) : (
              <p className="text-gray-500 animate-pulse font-mono mt-4">Handshake pending on :8081...</p>
            )}
          </section>

        </div>
      </main>
    </div>
  );
}

export default App;