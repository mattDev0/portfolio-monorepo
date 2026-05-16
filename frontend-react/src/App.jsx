import { useState, useEffect } from 'react';

function App() {
  // State to hold the data returning from our Rust API
  const [rustStatus, setRustStatus] = useState(null);

  // Fetch the data exactly once when the component mounts
  useEffect(() => {
    fetch('http://localhost:8080/api/status')
      .then(response => response.json())
      .then(data => setRustStatus(data))
      .catch(error => console.error("Error fetching from Rust API:", error));
  }, []);

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col items-center justify-center p-8">
      
      <header className="text-center mb-12">
        <h1 className="text-5xl font-bold text-blue-400 mb-4 tracking-tight">Odunayo Abraham</h1>
        <h2 className="text-2xl font-semibold text-gray-300 mb-6">DevOps Engineer & Software Developer</h2>
        <p className="max-w-2xl text-gray-400 leading-relaxed">
          Bridging low-level systems, high-performance backends, and modern web architecture.
        </p>
      </header>
      
      <main className="w-full max-w-4xl space-y-8">
        
        {/* Static Skills Section */}
        <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-gray-700">
          <h3 className="text-xl font-bold text-white mb-4 border-b border-gray-600 pb-2">Core Technologies</h3>
          <ul className="grid grid-cols-2 gap-4 text-gray-300">
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

        {/* Dynamic API Section */}
        <section className="bg-gray-800 p-8 rounded-xl shadow-lg border border-orange-500/30 relative overflow-hidden">
          {/* A glowing beacon to show the server is actively connected */}
          <div className="absolute top-0 right-0 p-4">
            <span className="flex h-3 w-3 relative">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-3 w-3 bg-orange-500"></span>
            </span>
          </div>

          <h3 className="text-xl font-bold text-orange-400 mb-4 border-b border-gray-600 pb-2">Live Rust Backend Status</h3>
          
          {rustStatus ? (
            <div className="font-mono text-sm space-y-2">
              <div className="flex justify-between border-b border-gray-700 pb-1">
                <span className="text-gray-400">Service:</span>
                <span className="text-green-400">{rustStatus.service}</span>
              </div>
              <div className="flex justify-between border-b border-gray-700 pb-1">
                <span className="text-gray-400">State:</span>
                <span className="text-blue-400">{rustStatus.status}</span>
              </div>
              <div className="flex justify-between border-b border-gray-700 pb-1">
                <span className="text-gray-400">Architecture Concept:</span>
                <span className="text-purple-400">{rustStatus.uptime_concept}</span>
              </div>
            </div>
          ) : (
            <p className="text-gray-500 animate-pulse font-mono">Initiating handshake with Axum server...</p>
          )}
        </section>

      </main>

    </div>
  );
}

export default App;