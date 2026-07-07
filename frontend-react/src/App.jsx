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
import CaseStudyDialog from './components/CaseStudyDialog';

// Components

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
      <CaseStudyDialog
        isOpen={showDevOpsCaseStudy}
        onClose={() => setShowDevOpsCaseStudy(false)}
        config={portfolioConfig}
      />
    </div>
  );
}

export default App;
