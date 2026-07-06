import { Settings, ExternalLink, Box } from 'lucide-react';
import ProjectCard from '../ui/ProjectCard';

export default function ProjectsSection({
  projects,
  selectedTech,
  setSelectedTech,
  setShowDevOpsCaseStudy
}) {
  // Extract all unique technology tags from projects
  const allTechTags = Array.from(
    new Set(projects.flatMap(p => p.tech))
  );

  const filteredProjects = selectedTech
    ? projects.filter(p => p.tech.includes(selectedTech))
    : projects;

  const devopsProject = projects.find(p => p.title === "DevOps Control Center");
  const isDevopsFeatured = filteredProjects.some(p => p.title === "DevOps Control Center");

  const otherProjects = filteredProjects.filter(p => p.title !== "DevOps Control Center");

  const handleTechClick = (tech) => {
    setSelectedTech(prev => (prev === tech ? null : tech));
  };

  return (
    <section id="projects" className="space-y-6">
      
      {/* Section Header and Filters */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 pb-4 border-b border-[var(--border-default)]">
        <h2 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-semibold text-[var(--fg-default)]">
          Featured Projects
        </h2>

        {/* Project Filter Controls */}
        <div className="flex flex-wrap gap-1.5" role="group" aria-label="Filter projects by technology">
          <button
            onClick={() => setSelectedTech(null)}
            aria-pressed={!selectedTech}
            className={`px-3 py-1 rounded-[var(--radius-sm)] text-xs font-semibold tracking-wide border transition-colors cursor-pointer focus-visible:outline-none ${
              !selectedTech
                ? 'bg-[var(--accent-primary)] text-white border-transparent'
                : 'bg-[var(--bg-elevated)] border-[var(--border-default)] text-[var(--fg-muted)] hover:text-[var(--fg-default)] hover:border-[var(--fg-subtle)]'
            }`}
          >
            All
          </button>
          {allTechTags.map(tag => {
            const isTagActive = selectedTech === tag;
            return (
              <button
                key={tag}
                onClick={() => setSelectedTech(isTagActive ? null : tag)}
                aria-pressed={isTagActive}
                className={`px-3 py-1 rounded-[var(--radius-sm)] text-xs font-semibold tracking-wide border transition-colors cursor-pointer focus-visible:outline-none ${
                  isTagActive
                    ? 'bg-[var(--accent-primary)] text-white border-transparent'
                    : 'bg-[var(--bg-elevated)] border-[var(--border-default)] text-[var(--fg-muted)] hover:text-[var(--fg-default)] hover:border-[var(--fg-subtle)]'
                }`}
              >
                {tag}
              </button>
            );
          })}
        </div>
      </div>

      {/* Screen Reader Announcements */}
      <div className="sr-only" aria-live="polite">
        {filteredProjects.length === 0 
          ? "No projects match the selected filter." 
          : `Showing ${filteredProjects.length} project${filteredProjects.length === 1 ? '' : 's'}.`}
      </div>

      {/* Projects Display */}
      {filteredProjects.length === 0 ? (
        <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-12 text-center flex flex-col items-center justify-center space-y-3">
          <Box className="w-8 h-8 text-[var(--fg-subtle)] opacity-50" />
          <h3 className="text-sm font-semibold text-[var(--fg-muted)]">No Projects Found</h3>
          <p className="text-xs text-[var(--fg-subtle)] max-w-xs leading-normal">
            No projects in the config database match the selected filter tag. Select "All" to restore projects view.
          </p>
        </div>
      ) : (
        <div className="space-y-6">
          
          {/* Featured Project Panel */}
          {isDevopsFeatured && devopsProject && (
            <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-6 md:p-8 relative overflow-hidden">
              <div className="flex flex-col md:flex-row md:items-start justify-between gap-6">
                
                <div className="flex-1 space-y-4">
                  <div className="flex items-center space-x-2.5">
                    <span className="bg-[var(--accent-primary)]/10 text-[var(--accent-primary)] border border-[var(--accent-primary)]/20 px-2.5 py-0.5 rounded-[var(--radius-sm)] text-[10px] font-bold tracking-wider uppercase">
                      Featured Project
                    </span>
                  </div>
                  
                  <h3 className="text-[var(--text-xl)] leading-[var(--lh-xl)] font-bold text-[var(--fg-default)]">
                    {devopsProject.title}
                  </h3>
                  
                  <p className="text-[var(--text-base)] leading-[var(--lh-base)] text-[var(--fg-muted)]">
                    {devopsProject.description}
                  </p>

                  <div className="flex flex-wrap gap-1.5 pt-2">
                    {devopsProject.tech.map((tech, i) => {
                      const isFilterActive = selectedTech === tech;
                      return (
                        <button
                          key={i}
                          onClick={() => handleTechClick(tech)}
                          aria-pressed={isFilterActive}
                          className={`text-[10px] px-2.5 py-0.5 rounded-[var(--radius-sm)] border font-medium transition-colors cursor-pointer ${
                            isFilterActive
                              ? 'bg-[var(--accent-primary)]/10 text-[var(--accent-primary)] border-[var(--accent-primary)]/20'
                              : 'bg-[var(--bg-elevated)] border-[var(--border-default)] text-[var(--fg-muted)] hover:text-[var(--fg-default)] hover:border-[var(--fg-subtle)]'
                          }`}
                        >
                          {tech}
                        </button>
                      );
                    })}
                  </div>
                </div>

                {/* Actions Panel */}
                <div className="flex flex-col justify-center space-y-3 w-full md:w-64 flex-shrink-0 md:pt-4">
                  <button
                    onClick={() => setShowDevOpsCaseStudy(true)}
                    className="inline-flex items-center justify-center space-x-2 px-5 py-3 bg-[var(--accent-primary)] hover:bg-[var(--accent-primary-hover)] text-white rounded-[var(--radius-md)] text-xs font-bold transition-colors w-full cursor-pointer focus-visible:outline-none"
                  >
                    <Settings className="w-4 h-4" />
                    <span>System Design & Demo</span>
                  </button>

                  <a
                    href={devopsProject.link}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="inline-flex items-center justify-center space-x-2 px-5 py-3 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] hover:bg-[var(--bg-surface)] text-[var(--fg-default)] rounded-[var(--radius-md)] text-xs font-bold transition-colors w-full text-center cursor-pointer focus-visible:outline-none"
                  >
                    <span>View Repository</span>
                    <ExternalLink className="w-3.5 h-3.5 text-[var(--fg-subtle)]" />
                  </a>
                </div>

              </div>
            </div>
          )}

          {/* Grid of Other Projects */}
          {otherProjects.length > 0 && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {otherProjects.map((project, index) => (
                <ProjectCard
                  key={index}
                  project={project}
                  selectedTech={selectedTech}
                  onTechClick={handleTechClick}
                />
              ))}
            </div>
          )}

        </div>
      )}

    </section>
  );
}
