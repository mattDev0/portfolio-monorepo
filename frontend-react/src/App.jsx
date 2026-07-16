import { useState, useEffect } from 'react';
import portfolioConfig from './config.json';
import GitHubActivity from './GitHubActivity';

// Components
import Sparkline from './components/Sparkline';
import TerminalSimulator from './components/TerminalSimulator';
import TopologyNode from './components/TopologyNode';
import SpotifyPlayer from './components/SpotifyPlayer';
import ThemeToggle from './components/ThemeToggle';

// Hooks
import useTelemetry from './hooks/useTelemetry';
import useSpotify from './hooks/useSpotify';

// Constants
import { TOPOLOGY_INFO } from './constants';

// Telemetry and routing updated for Traefik Ingress
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

  const [activeSection, setActiveSection] = useState('about');
  const [isScrolled, setIsScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 50);

      const sections = ['about', 'skills', 'projects', 'experience', 'infrastructure'];
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
    javaStatus,
    telemetryHistory,
    networkStatus,
    networkHistory
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
    <div className="min-h-screen bg-[var(--color-bg-primary)] text-[var(--color-text-primary)] flex flex-col items-center pt-12 pb-8 px-4 sm:px-6 lg:px-8" style={{ backgroundImage: 'var(--gradient-hero)' }}>

      {/* Sticky Navigation Bar */}
      <nav 
        className={`fixed top-0 left-0 right-0 z-40 border-b transition-all duration-300 ${isScrolled ? 'bg-[var(--color-nav-bg-scrolled)] backdrop-blur-md border-[var(--color-nav-border-scrolled)] py-4' : 'bg-transparent border-transparent py-6'}`}
        style={isScrolled ? { boxShadow: 'var(--color-nav-shadow)' } : undefined}
      >
        <div className="max-w-5xl mx-auto px-4 sm:px-8 flex items-center justify-between">
          <a href="#" className="font-bold text-lg bg-clip-text text-transparent tracking-tight" style={{ backgroundImage: 'var(--gradient-name)' }}>
            {portfolioConfig.name}
          </a>
          <div className="hidden md:flex items-center space-x-6">
            {['about', 'skills', 'projects', 'experience', 'infrastructure'].map((sec) => (
              <a
                key={sec}
                href={`#${sec}`}
                className={`text-xs font-bold tracking-wide uppercase transition-all duration-300 hover:text-[var(--color-hover-text)] hover:scale-105 ${activeSection === sec ? 'text-[var(--color-accent-emerald)] border-b border-[var(--color-accent-emerald)] pb-1' : 'text-[var(--color-text-tertiary)]'}`}
              >
                {sec}
              </a>
            ))}
            <div className="border-l border-[var(--color-border-primary)] pl-4 ml-2">
              <ThemeToggle />
            </div>
          </div>
          {/* Subtle Mobile indicators */}
          <div className="md:hidden flex items-center space-x-3">
            <div className="flex items-center space-x-2 bg-[var(--color-bg-surface)] px-3 py-1 rounded-full border border-[var(--color-border-primary)] text-[10px] text-[var(--color-accent-emerald)] font-bold uppercase tracking-wider font-mono">
              <span className="h-1.5 w-1.5 rounded-full bg-[var(--color-status-online)] animate-ping"></span>
              <span>{activeSection}</span>
            </div>
            <ThemeToggle />
          </div>
        </div>
      </nav>

      {/* Header Section */}
      <header className="text-center mb-10 md:mb-12 max-w-3xl w-full">
        {/* Avatar / Profile Picture */}
        <div className="mb-6 relative inline-block group">
          <div className="absolute inset-0 rounded-full blur opacity-40 group-hover:opacity-75 transition duration-500" style={{ backgroundImage: 'var(--gradient-glow)' }}></div>
          <img
            src="https://github.com/mattDev0.png"
            alt={portfolioConfig.name}
            className="relative w-24 h-24 sm:w-28 sm:h-28 rounded-full border-2 border-[var(--color-accent-emerald-border)] p-1 bg-[var(--color-bg-primary)] shadow-2xl transition-transform duration-500 group-hover:scale-105"
          />
        </div>

        <h1 className="text-4xl sm:text-5xl md:text-6xl font-extrabold bg-clip-text text-transparent mb-4 tracking-tight" style={{ backgroundImage: 'var(--gradient-name)' }}>
          {portfolioConfig.name}
        </h1>
        <h2 className="text-lg sm:text-xl font-medium text-[var(--color-text-tertiary)] mb-4 tracking-wide">{portfolioConfig.title}</h2>

        {/* Location and Remote Status */}
        <div className="flex items-center justify-center text-[var(--color-text-tertiary)] mb-6 text-sm">
          <svg className="w-4 h-4 mr-2 text-[var(--color-accent-emerald)]" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span className="font-medium text-[var(--color-text-secondary)]">{portfolioConfig.location}</span>

          {portfolioConfig.openToRemote && (
            <>
              <span className="mx-3 text-[var(--color-text-faint)]">•</span>
              <span className="bg-[var(--color-accent-emerald-bg)] text-[var(--color-accent-emerald)] border border-[var(--color-accent-emerald-border)] px-2 py-0.5 rounded-full text-xs font-semibold tracking-wider uppercase">
                Open to Remote
              </span>
            </>
          )}
        </div>

        {/* Tagline */}
        <p className="max-w-2xl mx-auto text-[var(--color-text-secondary)] leading-relaxed text-sm sm:text-base mb-8">
          {portfolioConfig.tagline}
        </p>

        {/* Contact Links */}
        <div className="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 max-w-sm sm:max-w-none mx-auto w-full sm:w-auto">
          <a
            href={portfolioConfig.githubUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center justify-center space-x-2 px-6 py-2.5 bg-[var(--color-accent-emerald-solid)] border border-[var(--color-accent-emerald-solid)] text-white hover:bg-[var(--color-accent-emerald-solid-hover)] rounded-xl transition-all duration-300 w-full sm:w-auto"
            style={{ boxShadow: 'var(--shadow-button)' }}
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path fillRule="evenodd" clipRule="evenodd" d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.464-1.11-1.464-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.579.688.481C19.138 20.161 22 16.418 22 12c0-5.523-4.477-10-10-10z" />
            </svg>
            <span className="font-semibold text-sm">GitHub</span>
          </a>
          <a
            href={`mailto:${portfolioConfig.email}`}
            className="flex items-center justify-center space-x-2 px-6 py-2.5 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] rounded-xl transition-all duration-300 w-full sm:w-auto"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <span className="font-semibold text-sm">Email Me</span>
          </a>
        </div>
      </header>
      <main className="w-full max-w-5xl space-y-16 md:space-y-20">

        {/* About the Developer */}
        <section id="about" className="reveal-in card-surface p-6 md:p-8 rounded-2xl flex flex-col justify-between transition-colors duration-300">
          <div>
            <h3 className="text-xl font-bold text-[var(--color-text-primary)] mb-4 border-b border-[var(--color-border-primary)] pb-3 tracking-wide">About the Developer</h3>
            <p className="text-[var(--color-text-secondary)] leading-relaxed text-sm whitespace-pre-line">
              {portfolioConfig.about}
            </p>

            {/* Highlight Badges */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
              {portfolioConfig.highlights && portfolioConfig.highlights.map((hl, i) => (
                <div key={i} className="hover-lift bg-[var(--color-bg-surface-elevated)] p-4 rounded-xl border border-[var(--color-border-primary)] hover:border-[var(--color-accent-emerald-border)] transition-all duration-300">
                  <div className="flex items-center space-x-2.5 mb-1.5">
                    <span className="text-lg">{hl.icon}</span>
                    <h4 className="text-xs font-bold text-[var(--color-text-primary)] tracking-wide">{hl.label}</h4>
                  </div>
                  <p className="text-[11px] text-[var(--color-text-tertiary)] leading-relaxed">{hl.detail}</p>
                </div>
              ))}
            </div>
          </div>
          <div className="mt-8 pt-6 border-t border-[var(--color-border-primary)]">
            <h4 className="text-xs font-bold text-[var(--color-accent-emerald)] uppercase tracking-widest mb-2">Education</h4>
            <div className="flex justify-between items-center">
              <span className="text-[var(--color-text-primary)] font-semibold">{portfolioConfig.education.degree}</span>
              <span className="text-[var(--color-accent-emerald)] font-semibold text-sm">{portfolioConfig.education.year}</span>
            </div>
            <div className="text-[var(--color-text-tertiary)] text-sm mt-1">{portfolioConfig.education.institution}</div>
          </div>
        </section>

        {/* Technical Skills Section */}
        <section id="skills" className="reveal-in card-surface p-6 md:p-8 rounded-2xl transition-colors duration-300">
          <h3 className="text-xl font-bold text-[var(--color-text-primary)] mb-6 border-b border-[var(--color-border-primary)] pb-3 tracking-wide">Technical Skills</h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-6">
            {portfolioConfig.skills && Object.entries(portfolioConfig.skills).map(([category, skills]) => (
              <div key={category} className="space-y-3">
                <h4 className="text-xs font-bold text-[var(--color-accent-emerald)] uppercase tracking-wider">{category}</h4>
                <div className="flex flex-wrap gap-2">
                  {skills.map((skill, index) => (
                    <span key={index} className="bg-[var(--color-accent-emerald-bg)] border border-[var(--color-accent-emerald-border)] text-[var(--color-text-secondary)] text-xs px-2.5 py-1 rounded-lg hover:border-[var(--color-accent-emerald)] transition-colors duration-200">
                      {skill}
                    </span>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Featured Projects Section */}
        <section id="projects" className="reveal-in space-y-6">
          <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-[var(--color-border-primary)] pb-3">
            <h3 className="text-2xl font-extrabold text-[var(--color-text-primary)] tracking-wide">Featured Projects</h3>

            {/* Project Filter Controls */}
            <div className="flex flex-wrap gap-2">
              <button
                onClick={() => setSelectedTech(null)}
                className={`px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer ${!selectedTech ? 'bg-[var(--color-accent-emerald-solid)] border-[var(--color-accent-emerald-solid)] text-white shadow-[0_0_8px_rgba(16,185,129,0.3)]' : 'bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)]'}`}
              >
                All
              </button>
              {allTechTags.map(tag => (
                <button
                  key={tag}
                  onClick={() => setSelectedTech(tag === selectedTech ? null : tag)}
                  className={`px-3 py-1 rounded-lg text-xs font-bold tracking-wide border transition-all duration-300 cursor-pointer ${tag === selectedTech ? 'bg-[var(--color-accent-emerald-solid)] border-[var(--color-accent-emerald-solid)] text-white shadow-[0_0_8px_rgba(16,185,129,0.3)]' : 'bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-tertiary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]'}`}
                >
                  {tag}
                </button>
              ))}
            </div>
          </div>

          {/* Featured Card or Grid Layout */}
          <div className="space-y-6">
            {/* Find if DevOps Control Center matches the current filter */}
            {filteredProjects.some(p => p.title === "DevOps Control Center") && (() => {
              const devopsProject = portfolioConfig.projects.find(p => p.title === "DevOps Control Center");
              return (
                <div 
                  className="hover-lift backdrop-blur-md p-6 md:p-8 rounded-2xl border-t-2 border-t-[var(--color-accent-emerald-solid)] border-x border-b border-[var(--color-border-primary)] relative overflow-hidden transition-all duration-300 group hover:border-[var(--color-accent-emerald-border)]"
                  style={{ background: 'linear-gradient(to bottom right, var(--color-accent-emerald-bg), var(--color-bg-surface), var(--color-bg-primary))', boxShadow: 'var(--shadow-card)' }}
                >
                  <div className="absolute top-0 right-0 w-32 h-32 bg-[var(--color-accent-emerald-bg)] rounded-full blur-2xl -z-10"></div>
                  <div className="flex flex-col md:flex-row md:items-center justify-between gap-6">
                    <div className="flex-1 space-y-4">
                      <div className="flex items-center space-x-2">
                        <span className="bg-[var(--color-accent-emerald-bg)] text-[var(--color-accent-emerald)] border border-[var(--color-accent-emerald-border)] px-2.5 py-0.5 rounded-full text-[10px] font-bold tracking-wider uppercase">
                          Featured Project
                        </span>
                        <span className="text-xl">⚙️</span>
                      </div>
                      <h4 className="text-xl md:text-2xl font-bold text-[var(--color-text-primary)] group-hover:text-[var(--color-accent-emerald)] transition-colors tracking-wide">
                        {devopsProject.title}
                      </h4>
                      <p className="text-[var(--color-text-secondary)] text-sm md:text-base leading-relaxed">
                        {devopsProject.description}
                      </p>

                      <div className="flex flex-wrap gap-1.5 pt-2">
                        {devopsProject.tech.map((tech, i) => (
                          <span
                            key={i}
                            onClick={() => setSelectedTech(tech === selectedTech ? null : tech)}
                            className={`border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 ${tech === selectedTech ? 'bg-[var(--color-bg-tag-active)] border-[var(--color-accent-emerald-solid)] text-[var(--color-accent-emerald)]' : 'bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]'}`}
                          >
                            {tech}
                          </span>
                        ))}
                      </div>
                    </div>

                    <div className="flex flex-col justify-center space-y-3 w-full md:w-64 flex-shrink-0">
                      <button
                        onClick={() => setShowDevOpsCaseStudy(true)}
                        className="px-5 py-3 bg-[var(--color-accent-emerald-solid)] hover:bg-[var(--color-accent-emerald-solid-hover)] text-white rounded-xl text-xs font-bold transition-all duration-300 w-full cursor-pointer flex items-center justify-center space-x-1.5"
                        style={{ boxShadow: 'var(--shadow-button)' }}
                      >
                        <span>⚙️</span>
                        <span>System Design & Demo</span>
                      </button>

                      <a
                        href={devopsProject.link}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="px-5 py-3 bg-[var(--color-hover-bg)] border border-[var(--color-border-primary)] hover:bg-[var(--color-hover-bg)] hover:border-[var(--color-border-secondary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] rounded-xl text-xs font-bold transition-all duration-300 w-full text-center flex items-center justify-center space-x-1.5"
                      >
                        <span>View Repository</span>
                        <span>→</span>
                      </a>
                    </div>
                  </div>
                </div>
              );
            })()}

            {/* Other Projects Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {filteredProjects
                .filter(p => p.title !== "DevOps Control Center")
                .map((project, index) => {
                  let accentClass = "border-t-[var(--color-accent-teal)]";
                  let emoji = "📁";
                  if (project.title.includes("Flutter")) {
                    accentClass = "border-t-[var(--color-accent-orange)]";
                    emoji = "📱";
                  } else if (project.title.includes("Portfolio")) {
                    accentClass = "border-t-[var(--color-accent-emerald-solid)]";
                    emoji = "🌐";
                  }

                  return (
                    <div 
                      key={index} 
                      className={`hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border-t-2 ${accentClass} border-x border-b transition-all duration-500 flex flex-col justify-between group ${selectedTech && project.tech.includes(selectedTech) ? 'border-[var(--color-accent-emerald-border)]' : 'border-[var(--color-border-primary)] hover:border-[var(--color-accent-emerald-border)]'}`}
                      style={{ boxShadow: selectedTech && project.tech.includes(selectedTech) ? 'var(--shadow-card-hover)' : 'var(--shadow-card)' }}
                    >
                      <div>
                        <div className="flex items-center space-x-2 mb-2">
                          <span className="text-lg">{emoji}</span>
                          <h4 className="text-lg font-bold text-[var(--color-text-primary)] group-hover:text-[var(--color-accent-emerald)] transition-colors tracking-wide">{project.title}</h4>
                        </div>
                        <p className="text-[var(--color-text-tertiary)] text-sm leading-relaxed mb-6">{project.description}</p>
                      </div>
                      <div>
                        <div className="flex flex-wrap gap-1.5 mb-5">
                          {project.tech.map((tech, i) => (
                            <span
                              key={i}
                              onClick={() => setSelectedTech(tech === selectedTech ? null : tech)}
                              className={`border text-[10px] px-2.5 py-0.5 rounded-md font-semibold tracking-wide cursor-pointer transition-all duration-200 ${tech === selectedTech ? 'bg-[var(--color-bg-tag-active)] border-[var(--color-accent-emerald-solid)] text-[var(--color-accent-emerald)]' : 'bg-[var(--color-bg-tag)] border-[var(--color-border-primary)] text-[var(--color-text-secondary)] hover:text-[var(--color-hover-text)] hover:border-[var(--color-border-secondary)]'}`}
                            >
                              {tech}
                            </span>
                          ))}
                        </div>

                        <a
                          href={project.link}
                          target="_blank"
                          rel="noopener noreferrer"
                          className="px-4 py-2 border border-[var(--color-accent-emerald-border)] rounded-lg text-xs font-semibold text-[var(--color-accent-emerald)] hover:text-[var(--color-accent-emerald-solid-hover)] hover:border-[var(--color-accent-emerald)] inline-flex items-center transition-all duration-300 w-full justify-center"
                        >
                          View Repository <span className="ml-1">→</span>
                        </a>
                      </div>
                    </div>
                  );
                })}
            </div>
          </div>
        </section>

        {/* Professional Experience Section */}
        <section id="experience" className="reveal-in card-surface p-6 md:p-8 rounded-2xl transition-colors duration-300">
          <h3 className="text-xl font-bold text-[var(--color-text-primary)] mb-6 border-b border-[var(--color-border-primary)] pb-3 tracking-wide">Professional Experience</h3>
          <div className="space-y-8">
            {portfolioConfig.experience.map((exp, index) => (
              <div key={index} className="relative pl-8 border-l-2 border-[var(--color-accent-emerald-border)] hover:border-[var(--color-accent-emerald)] transition-colors duration-300">
                <div className="absolute w-4 h-4 bg-[var(--color-status-online)] rounded-full -left-[9px] top-1 border-2 border-[var(--color-bg-primary)] shadow-[0_0_8px_rgba(16,185,129,0.5)]"></div>
                <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-1 mb-2">
                  <h4 className="text-[var(--color-text-primary)] font-bold text-base tracking-wide">{exp.role}</h4>
                  <p className="text-[var(--color-accent-emerald)] text-xs sm:text-sm font-semibold">{exp.company}</p>
                </div>
                <p className="text-[var(--color-text-tertiary)] text-sm leading-relaxed">{exp.description}</p>
              </div>
            ))}
          </div>
        </section>

        {/* Live Infrastructure Dashboard */}
        <section id="infrastructure" className="reveal-in space-y-8">
          <div className="border-b border-[var(--color-border-primary)] pb-3">
            <span className="text-[10px] font-bold text-[var(--color-accent-emerald)] uppercase tracking-widest font-mono">Systems Observability</span>
            <h3 className="text-2xl font-extrabold text-[var(--color-text-primary)] tracking-wide mt-1">Live Infrastructure Dashboard</h3>
            <p className="text-xs text-[var(--color-text-tertiary)] mt-1">Real-time health telemetry and network architecture mapping my deployed systems.</p>
          </div>

          {/* Architecture Topology - Full Width */}
          <div className="bg-[var(--color-bg-surface-deep)] backdrop-blur-md p-6 md:p-8 rounded-2xl border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] transition-colors duration-300">
            <div className="border-b border-[var(--color-border-primary)] pb-3 mb-6">
              <span className="text-[10px] font-bold text-[var(--color-accent-emerald)] uppercase tracking-widest font-mono">Deployment Topology</span>
              <h4 className="text-lg font-bold text-[var(--color-text-primary)] tracking-wide mt-0.5">Microservices Architecture</h4>
            </div>

            {/* Topology diagram wrapper */}
            <div className="flex flex-col lg:flex-row gap-8 items-center lg:items-stretch">

              {/* Left/Main column: The diagram */}
              <div className="flex-grow w-full flex flex-col gap-6 justify-center">

                <div className="flex flex-col md:flex-row items-center md:items-stretch gap-4 justify-between">

                  {/* 1. Client Card */}
                  <div className="flex-1 w-full flex flex-col justify-center">
                    <TopologyNode id="client" icon="🌐" title="Client Browser" tech="React 19 + Vite" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                  </div>

                  {/* Arrow Client -> Nginx */}
                  <div className="flex items-center justify-center text-[var(--color-text-muted)] font-mono text-xs py-1 md:py-0">
                    <span className="md:hidden">⬇️</span>
                    <span className="hidden md:inline text-[var(--color-accent-emerald)] opacity-40">── HTTPS ──▶</span>
                  </div>

                  {/* 2. Nginx Card */}
                  <div className="flex-1 w-full flex flex-col justify-center">
                    <TopologyNode id="nginx" icon="🛡️" title="GCP Ingress" tech="HTTPS Gateway" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                  </div>

                  {/* Arrow Nginx -> K8s Namespace */}
                  <div className="flex items-center justify-center text-[var(--color-text-muted)] font-mono text-xs py-1 md:py-0">
                    <span className="md:hidden">⬇️</span>
                    <span className="hidden md:inline text-[var(--color-accent-emerald)] opacity-40">── Proxy ──▶</span>
                  </div>

                  {/* 3. Google Cloud Run (Serverless) */}
                  <div
                    onMouseEnter={() => setHoveredTopologyNode('k8s')}
                    onMouseLeave={() => setHoveredTopologyNode(null)}
                    className={`flex-[2] w-full p-4 rounded-xl border border-dashed transition-all duration-300 ${hoveredTopologyNode === 'k8s' ? 'border-[var(--color-accent-teal)] bg-[var(--color-accent-emerald-bg)] shadow-[0_0_15px_rgba(20,184,166,0.1)]' : 'border-[var(--color-border-secondary)] bg-[var(--color-bg-surface-deep)]'}`}
                  >
                    <div className="flex justify-between items-center mb-3">
                      <span className="text-[8px] font-bold text-[var(--color-text-muted)] uppercase tracking-widest font-mono">Platform: Google Cloud</span>
                      <span className="text-[8px] font-mono text-[var(--color-accent-teal)] bg-[var(--color-accent-emerald-bg)] px-1 py-0.5 rounded">🚀 GCP Cloud Run</span>
                    </div>

                    <div className="grid grid-cols-3 gap-2">
                      {/* Service 1: Frontend */}
                      <TopologyNode id="frontend" icon="⚛️" title="Frontend Service" tech="Cloud Run" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                      {/* Service 2: Rust API */}
                      <TopologyNode id="rust" icon="🦀" title="Rust Service" tech="Cloud Run" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                      {/* Service 3: Java API */}
                      <TopologyNode id="java" icon="☕" title="Java Service" tech="Cloud Run" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                    </div>
                  </div>

                </div>

                {/* Connections from K8s to external services */}
                <div className="flex flex-col md:flex-row gap-4 justify-between items-stretch">

                  {/* Arrow Rust -> Spotify */}
                  <div className="flex-1 bg-[var(--color-bg-surface-deep)] border border-[var(--color-border-primary)] rounded-xl p-3.5 flex flex-col justify-between hover:border-[var(--color-accent-orange-border)] transition-all duration-300">
                    <div className="flex justify-between items-center text-[8px] text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
                      <span>Rust Gateway</span>
                      <span className="text-[var(--color-accent-orange)] font-mono">Axum Outbound</span>
                    </div>
                    <div className="flex items-center justify-between gap-2">
                      <span className="text-xs font-bold text-[var(--color-text-secondary)]">Rust API</span>
                      <span className="text-[var(--color-accent-orange)] font-mono text-[10px]">── OAuth ──▶</span>
                      <TopologyNode id="spotify" icon="🎵" title="Spotify API" tech="v1 Player Web Service" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                    </div>
                  </div>

                  {/* Arrow Java -> GitHub */}
                  <div className="flex-1 bg-[var(--color-bg-surface-deep)] border border-[var(--color-border-primary)] rounded-xl p-3.5 flex flex-col justify-between hover:border-[var(--color-accent-emerald-border)] transition-all duration-300">
                    <div className="flex justify-between items-center text-[8px] text-[var(--color-text-muted)] uppercase tracking-wider mb-2">
                      <span>Java Caching</span>
                      <span className="text-[var(--color-accent-emerald)] font-mono">Spring cacheable</span>
                    </div>
                    <div className="flex items-center justify-between gap-2">
                      <span className="text-xs font-bold text-[var(--color-text-secondary)]">Java API</span>
                      <span className="text-[var(--color-accent-emerald)] font-mono text-[10px]">── REST ──▶</span>
                      <TopologyNode id="github" icon="🐙" title="GitHub API" tech="v3 Public REST" hoveredTopologyNode={hoveredTopologyNode} setHoveredTopologyNode={setHoveredTopologyNode} />
                    </div>
                  </div>

                </div>

              </div>

              {/* Right/Inspector column: The Topology Inspector */}
              <div className="w-full lg:w-80 flex-shrink-0 bg-[var(--color-bg-surface-elevated)] border border-[var(--color-border-primary)] rounded-xl p-5 flex flex-col justify-between relative overflow-hidden">
                <div className="absolute top-0 right-0 w-24 h-24 bg-[var(--color-accent-emerald-bg)] rounded-full blur-2xl -z-10"></div>

                <div>
                  <div className="flex items-center space-x-2 border-b border-[var(--color-border-primary)] pb-2 mb-3">
                    <span className="flex h-2 w-2 relative">
                      <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--color-status-online)] opacity-75"></span>
                      <span className="relative inline-flex rounded-full h-2 w-2 bg-[var(--color-status-online)]"></span>
                    </span>
                    <h4 className="text-xs font-bold text-[var(--color-text-secondary)] uppercase tracking-wider">Topology Inspector</h4>
                  </div>

                  {hoveredTopologyNode ? (
                    <div className="space-y-3.5 animate-fadeIn">
                      <div className="flex justify-between items-center">
                        <span className="text-sm font-bold text-[var(--color-accent-emerald)]">{TOPOLOGY_INFO[hoveredTopologyNode].title}</span>
                        <span className="bg-[var(--color-accent-emerald-bg)] border border-[var(--color-accent-emerald-border)] text-[var(--color-accent-emerald)] text-[8px] px-1.5 py-0.5 rounded font-mono uppercase tracking-widest">
                          {TOPOLOGY_INFO[hoveredTopologyNode].badge}
                        </span>
                      </div>

                      <div className="font-mono text-[10px] space-y-1 bg-[var(--color-bg-surface-inset)] p-2 rounded-lg border border-[var(--color-border-primary)]">
                        <div className="flex justify-between">
                          <span className="text-[var(--color-text-muted)]">ENGINE:</span>
                          <span className="text-[var(--color-text-secondary)]">{TOPOLOGY_INFO[hoveredTopologyNode].tech}</span>
                        </div>
                        <div className="flex justify-between mt-1">
                          <span className="text-[var(--color-text-muted)]">PORT/PROTO:</span>
                          <span className="text-[var(--color-accent-emerald)] font-semibold">{TOPOLOGY_INFO[hoveredTopologyNode].protocol}</span>
                        </div>
                      </div>

                      <p className="text-[var(--color-text-tertiary)] text-xs leading-relaxed">
                        {TOPOLOGY_INFO[hoveredTopologyNode].description}
                      </p>
                    </div>
                  ) : (
                    <div className="text-center py-10 flex flex-col items-center justify-center space-y-3">
                      <span className="text-2xl opacity-40 animate-bounce">🔍</span>
                      <p className="text-[var(--color-text-muted)] text-xs leading-relaxed font-mono">
                        Hover over any node or container in the diagram to inspect microservice details, proxy routes, and deployment states.
                      </p>
                    </div>
                  )}
                </div>

                <div className="mt-6 pt-3 border-t border-[var(--color-border-primary)] text-[9px] text-[var(--color-text-muted)] font-mono flex items-center justify-between">
                  <span>DEPLOYMENT: live</span>
                  <span>VM: azure-standard-b1s</span>
                </div>
              </div>

            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">

            {/* Left/Spans 2 columns: GitHub Commits */}
            <div className="lg:col-span-2 flex flex-col gap-8">
              {/* GitHub Activity */}
              <GitHubActivity />
            </div>

            {/* Right column: Live system telemetry & Spotify */}
            <div className="lg:col-span-1 flex flex-col gap-6 font-mono">

              {/* Rust Engine */}
              <section className="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-orange-border)] hover:border-[var(--color-accent-orange)] relative overflow-hidden transition-all duration-300 group" style={{ boxShadow: 'var(--shadow-card)' }}>
                <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                  <span className="flex h-3 w-3 relative">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400 opacity-75"></span>
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-orange-400/40 opacity-40 delay-300"></span>
                    <span className="relative inline-flex rounded-full h-3 w-3 bg-orange-500 border border-slate-900 shadow-[0_0_8px_#f97316]"></span>
                  </span>
                  <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-orange-border)] p-2 text-center text-[10px] text-[var(--color-accent-orange)] font-mono shadow-xl z-20">
                    Live Axum Service
                  </div>
                </div>
                <h3 className="text-lg font-bold text-[var(--color-accent-orange)] mb-0 tracking-wide font-sans">Rust Engine</h3>
                <p className="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">Low-level OS telemetry & Spotify API gateway</p>

                {rustStatus ? (
                  <div className="font-mono text-xs space-y-4 mt-2">
                    <div className="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">OS</span>
                      <span className="text-[var(--color-text-primary)] font-medium">{rustStatus.os_info}</span>
                    </div>

                    <div className="border-b border-[var(--color-border-primary)] pb-2">
                      <div className="flex justify-between items-center mb-1.5">
                        <span className="text-[var(--color-text-muted)] uppercase text-[10px]">CPU Utilization</span>
                        <div className="flex items-center space-x-3">
                          <Sparkline data={telemetryHistory.map(h => h.cpu)} color="#f97316" max={100} />
                          <span className="text-[var(--color-accent-orange)] font-semibold">
                            {rustStatus.cpu_usage_percent !== undefined ? `${rustStatus.cpu_usage_percent.toFixed(1)}%` : "0%"}
                          </span>
                        </div>
                      </div>
                      <div className="w-full h-1.5 bg-[var(--color-bg-progress-track)] rounded-full overflow-hidden">
                        <div
                          className="h-full bg-gradient-to-r from-orange-500 to-amber-400 transition-all duration-1000 ease-out shadow-[0_0_8px_#f97316]"
                          style={{ width: `${rustStatus.cpu_usage_percent || 0}%` }}
                        ></div>
                      </div>
                    </div>

                    <div className="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">Threads</span>
                      <span className="text-[var(--color-accent-orange)] font-semibold">{rustStatus.cpu_core_count} Logical Cores</span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">Memory</span>
                      <div className="flex items-center space-x-3">
                        <Sparkline data={telemetryHistory.map(h => h.memory)} color="#f59e0b" max={100} />
                        <span className="text-[var(--color-accent-amber)] font-semibold">
                          {rustStatus.memory_used_mb} MB / {rustStatus.memory_total_mb} MB
                        </span>
                      </div>
                    </div>
                  </div>
                ) : (
                  <p className="text-[var(--color-text-muted)] animate-pulse font-mono text-xs mt-2">Handshake pending...</p>
                )}
              </section>

              {/* Java Infrastructure */}
              <section className="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-emerald-border)] hover:border-[var(--color-accent-emerald)] relative overflow-hidden transition-all duration-300 group" style={{ boxShadow: 'var(--shadow-card)' }}>
                <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                  <span className="flex h-3 w-3 relative">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400/40 opacity-40 delay-300"></span>
                    <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500 shadow-[0_0_8px_#10b981]"></span>
                  </span>
                  <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-emerald-border)] p-2 text-center text-[10px] text-[var(--color-accent-emerald)] font-mono shadow-xl z-20">
                    Live Spring Service
                  </div>
                </div>
                <h3 className="text-lg font-bold text-[var(--color-accent-emerald)] mb-0 tracking-wide font-sans">Java Infrastructure</h3>
                <p className="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">Spring Cache engine driving GitHub API events</p>

                {javaStatus ? (
                  <div className="font-mono text-xs space-y-4 mt-2">
                    <div className="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">Version</span>
                      <span className="text-[var(--color-text-primary)] font-medium">{javaStatus.engine}</span>
                    </div>
                    <div className="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">Uptime</span>
                      <span className="text-[var(--color-accent-emerald)] font-semibold">{javaStatus.uptime_hours}h {javaStatus.uptime_minutes}m</span>
                    </div>
                    <div className="flex justify-between border-b border-[var(--color-border-primary)] pb-2">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">Active Threads</span>
                      <span className="text-[var(--color-accent-emerald)] font-semibold">{javaStatus.active_threads} Threads</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-[var(--color-text-muted)] uppercase text-[10px]">JVM Memory</span>
                      <span className="text-[var(--color-accent-emerald)] font-semibold">{javaStatus.jvm_memory_used_mb} MB / {javaStatus.jvm_memory_total_mb} MB</span>
                    </div>
                  </div>
                ) : (
                  <p className="text-[var(--color-text-muted)] animate-pulse font-mono text-xs mt-2">Handshake pending...</p>
                )}
              </section>

              {/* Network Telemetry */}
              <section className="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-emerald-border)] hover:border-[var(--color-accent-emerald)] relative overflow-hidden transition-all duration-300 group" style={{ boxShadow: 'var(--shadow-card)' }}>
                <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
                  <span className="flex h-3 w-3 relative">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400/40 opacity-40 delay-300"></span>
                    <span className="relative inline-flex rounded-full h-3 w-3 bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]"></span>
                  </span>
                  <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-emerald-border)] p-2 text-center text-[10px] text-[var(--color-accent-emerald)] font-mono shadow-xl z-20">
                    Blackbox Exporter
                  </div>
                </div>
                <h3 className="text-lg font-bold text-[var(--color-accent-emerald)] mb-0 tracking-wide font-sans">Network Telemetry</h3>
                <p className="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">Synthetic latency probes via ICMP (Ping)</p>

                {networkStatus ? (
                  <div className="font-mono text-xs space-y-4 mt-2">
                    {/* Google DNS */}
                    <div className="flex justify-between items-center border-b border-[var(--color-border-primary)] pb-2">
                      <div className="flex items-center space-x-2">
                        <span className={`h-1.5 w-1.5 rounded-full ${networkStatus.google_dns.status === 'online' ? 'bg-[var(--color-status-online)] shadow-[0_0_6px_#10b981]' : 'bg-[var(--color-status-error)] animate-pulse'}`}></span>
                        <span className="text-[var(--color-text-primary)]">{networkStatus.google_dns.name}</span>
                        <span className="text-[9px] text-[var(--color-text-muted)]">({networkStatus.google_dns.target})</span>
                      </div>
                      <div className="flex items-center space-x-3">
                        <Sparkline data={networkHistory.map(h => h.google_dns)} color="rgba(16,185,129,0.5)" />
                        <span className="text-[var(--color-accent-emerald)] font-semibold w-14 text-right">{networkStatus.google_dns.latency_ms} ms</span>
                      </div>
                    </div>

                    {/* Cloudflare DNS */}
                    <div className="flex justify-between items-center border-b border-[var(--color-border-primary)] pb-2">
                      <div className="flex items-center space-x-2">
                        <span className={`h-1.5 w-1.5 rounded-full ${networkStatus.cloudflare_dns.status === 'online' ? 'bg-[var(--color-status-online)] shadow-[0_0_6px_#10b981]' : 'bg-[var(--color-status-error)] animate-pulse'}`}></span>
                        <span className="text-[var(--color-text-primary)]">{networkStatus.cloudflare_dns.name}</span>
                        <span className="text-[9px] text-[var(--color-text-muted)]">({networkStatus.cloudflare_dns.target})</span>
                      </div>
                      <div className="flex items-center space-x-3">
                        <Sparkline data={networkHistory.map(h => h.cloudflare_dns)} color="rgba(16,185,129,0.5)" />
                        <span className="text-[var(--color-accent-emerald)] font-semibold w-14 text-right">{networkStatus.cloudflare_dns.latency_ms} ms</span>
                      </div>
                    </div>

                    {/* Riot Games NA */}
                    <div className="flex justify-between items-center">
                      <div className="flex items-center space-x-2">
                        <span className={`h-1.5 w-1.5 rounded-full ${networkStatus.riot_games.status === 'online' ? 'bg-[var(--color-status-online)] shadow-[0_0_6px_#10b981]' : 'bg-[var(--color-status-error)] animate-pulse'}`}></span>
                        <span className="text-[var(--color-text-primary)]">{networkStatus.riot_games.name}</span>
                        <span className="text-[9px] text-[var(--color-text-muted)]">({networkStatus.riot_games.target})</span>
                      </div>
                      <div className="flex items-center space-x-3">
                        <Sparkline data={networkHistory.map(h => h.riot_games)} color="rgba(16,185,129,0.5)" />
                        <span className="text-[var(--color-accent-emerald)] font-semibold w-14 text-right">{networkStatus.riot_games.latency_ms} ms</span>
                      </div>
                    </div>
                  </div>
                ) : (
                  <p className="text-[var(--color-text-muted)] animate-pulse font-mono text-xs mt-2">Ping diagnostics pending...</p>
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

          </div>
        </section>
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
          <span className="text-teal-400 font-semibold font-mono">React</span>
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
            <div className="absolute top-0 left-1/4 w-72 h-72 bg-emerald-600/10 rounded-full blur-3xl -z-10"></div>
            <div className="absolute bottom-0 right-1/4 w-72 h-72 bg-emerald-600/5 rounded-full blur-3xl -z-10"></div>

            {/* Header */}
            <div className="flex justify-between items-start border-b border-white/5 pb-4 mb-6">
              <div>
                <span className="text-[10px] font-bold text-emerald-400 uppercase tracking-widest">Case Study & Architecture</span>
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
                  The infrastructure operates inside the K3s namespace <code className="text-emerald-400 font-mono text-[10px] bg-emerald-950/40 px-1 py-0.5 rounded">devops</code> behind an Nginx reverse proxy.
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
                  <div className="flex flex-col items-center justify-between bg-teal-950/20 p-3 rounded-lg border border-teal-500/20 text-center shadow-md">
                    <span className="text-xl">☸️</span>
                    <span className="text-[10px] font-bold text-teal-400 uppercase tracking-widest mt-1">K3s Cluster API</span>
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
                className="w-full sm:w-auto px-5 py-2.5 bg-emerald-600 hover:bg-emerald-500 text-white font-bold rounded-xl text-xs text-center transition-colors shadow-lg shadow-emerald-900/20 cursor-pointer flex items-center justify-center space-x-1.5"
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
