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
import Footer from './components/layout/Footer';

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
    <div className="min-h-screen text-[var(--fg-default)] flex flex-col items-center pt-12 pb-8 px-4 sm:px-6 lg:px-8">
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
      <Footer config={portfolioConfig} />

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
