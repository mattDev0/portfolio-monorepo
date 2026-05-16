function App() {
  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col items-center justify-center p-8">
      
      <header className="text-center mb-12">
        <h1 className="text-5xl font-bold text-blue-400 mb-4 tracking-tight">Odunayo Abraham</h1>
        <h2 className="text-2xl font-semibold text-gray-300 mb-6">DevOps Engineer & Software Developer</h2>
        <p className="max-w-2xl text-gray-400 leading-relaxed">
          Bridging low-level systems, high-performance backends, and modern web architecture.
        </p>
      </header>
      
      <main className="w-full max-w-4xl">
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
      </main>

    </div>
  )
}

export default App