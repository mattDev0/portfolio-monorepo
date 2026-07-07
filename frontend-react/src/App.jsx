import { useState, useEffect } from 'react';
import portfolioConfig from './config.json';
import SiteHeader from './components/layout/SiteHeader';
import HeroSection from './components/sections/HeroSection';
import AboutSection from './components/sections/AboutSection';
import SkillsSection from './components/sections/SkillsSection';
import ProjectsSection from './components/sections/ProjectsSection';
import ExperienceSection from './components/sections/ExperienceSection';
import ContactSection from './components/sections/ContactSection';
import TelemetrySection from './components/sections/TelemetrySection';

// Components
import TerminalSimulator from './components/TerminalSimulator';

// Hooks
import useTelemetry from './hooks/useTelemetry';
import useSpotify from './hooks/useSpotify';

// Telemetry and routing updated for Traefik Ingress
function App() {
  const [selectedTech, setSelectedTech] = useState(null);
  const [showDevOpsCaseStudy, setShowDevOpsCaseStudy] = useState(false);
  const [selectedTopologyNode, setSelectedTopologyNode] = useState(null);
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

  const [activeSection, setActiveSection] = useState('about');

  useEffect(() => {
    const handleScroll = () => {

      const sections = ['about', 'projects', 'skills', 'experience', 'infrastructure', 'contact'];
      const scrollPosition = window.scrollY + 200;

      for (const section of sections) {
        const el = document.getElementById(section);
        if (el) {
          const top = el.offsetTop;
          const height = el.offsetHeight;
          if (scrollPosition >= top && scrollPosition < top + height) {
            setActiveSection(section);
            break;
          }
        }
      }
    };

    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  // IntersectionObserver for scroll-reveal animations
  useEffect(() => {
    const revealElements = document.querySelectorAll('.reveal-in');
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('active');
          }
        });
      },
      { threshold: 0.05, rootMargin: '0px 0px -50px 0px' }
    );

    revealElements.forEach((el) => observer.observe(el));
    return () => observer.disconnect();
  }, []);

  const {
    rustStatus,
    rustError,
    javaStatus,
    javaError,
    telemetryHistory,
    networkStatus,
    networkError,
    networkHistory
  } = useTelemetry(isVisible);

  const {
    spotifyData,
    localProgressMs,
    progressPercent,
    formatTime
  } = useSpotify(isVisible);


  return (
    <div className="min-h-screen bg-[#0b0f19] text-gray-100 flex flex-col items-center pt-12 pb-8 px-4 sm:px-6 lg:px-8 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-blue-950/20 via-[#0b0f19] to-[#0b0f19]">
      <SiteHeader activeSection={activeSection} siteName={portfolioConfig.name} />

      <HeroSection config={portfolioConfig} />
      <main id="main-content" className="w-full max-w-5xl space-y-16 md:space-y-20">

        {/* About the Developer */}
        <AboutSection config={portfolioConfig} />

        {/* Technical Skills Section */}
        <SkillsSection skills={portfolioConfig.skills} />

        {/* Featured Projects Section */}
        <ProjectsSection
          projects={portfolioConfig.projects}
          selectedTech={selectedTech}
          setSelectedTech={setSelectedTech}
          setShowDevOpsCaseStudy={setShowDevOpsCaseStudy}
        />

        {/* Professional Experience Section */}
        <ExperienceSection experience={portfolioConfig.experience} />

        {/* Live Infrastructure Dashboard */}
        <TelemetrySection
          selectedTopologyNode={selectedTopologyNode}
          setSelectedTopologyNode={setSelectedTopologyNode}
          rustStatus={rustStatus}
          rustError={rustError}
          javaStatus={javaStatus}
          javaError={javaError}
          networkStatus={networkStatus}
          networkError={networkError}
          telemetryHistory={telemetryHistory}
          networkHistory={networkHistory}
          spotifyData={spotifyData}
          progressPercent={progressPercent}
          localProgressMs={localProgressMs}
          formatTime={formatTime}
        />

        {/* Contact Section */}
        <ContactSection config={portfolioConfig} />
      </main>

      {/* Footer Section */}
      <footer className="w-full max-w-5xl mt-16 pt-8 pb-4 border-t border-white/5 flex flex-col md:flex-row items-center justify-between gap-4 text-xs text-gray-500">
        <div className="flex items-center space-x-2">
          <span>&copy; {new Date().getFullYear()} {portfolioConfig.name}. All rights reserved.</span>
        </div>
        <div className="flex items-center space-x-1 text-gray-600">
          <span>Architected with</span>
          <span className="text-orange-500/80 font-semibold font-mono">Rust</span>
          <span>&bull;</span>
          <span className="text-emerald-500/80 font-semibold font-mono">Spring Boot</span>
          <span>&bull;</span>
          <span className="text-blue-400 font-semibold font-mono">React</span>
        </div>
        <div className="flex items-center space-x-4">
          <a href={portfolioConfig.githubUrl} target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors duration-200" title="GitHub">
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path fillRule="evenodd" clipRule="evenodd" d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.464-1.11-1.464-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.579.688.481C19.138 20.161 22 16.418 22 12c0-5.523-4.477-10-10-10z" />
            </svg>
          </a>
          <a href={`mailto:${portfolioConfig.email}`} className="hover:text-white transition-colors duration-200" title="Email">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </a>
        </div>
      </footer>

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
