export default function ProjectCard({ project, selectedTech, onTechClick }) {
  return (
    <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-5 flex flex-col justify-between h-full">
      <div>
        <h3 className="text-[var(--text-md)] font-semibold text-[var(--fg-default)] mb-2">
          {project.title}
        </h3>
        <p className="text-[var(--text-sm)] text-[var(--fg-muted)] leading-relaxed mb-4">
          {project.description}
        </p>
      </div>

      <div className="space-y-4">
        {/* Tech tags */}
        <div className="flex flex-wrap gap-1.5">
          {project.tech.map((tech, i) => {
            const isFilterActive = selectedTech === tech;
            return (
              <button
                key={i}
                onClick={() => onTechClick(tech)}
                aria-pressed={isFilterActive}
                className={`text-[10px] px-2 py-0.5 rounded-[var(--radius-sm)] border font-medium transition-colors cursor-pointer ${
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

        {/* Repository link */}
        <a
          href={project.link}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center justify-center space-x-1.5 px-4 py-2 border border-[var(--border-default)] bg-[var(--bg-elevated)] hover:bg-[var(--bg-surface)] text-[var(--fg-default)] hover:border-[var(--fg-subtle)] rounded-[var(--radius-md)] text-xs font-semibold transition-colors focus-visible:outline-none w-full text-center cursor-pointer"
        >
          <span>View Repository</span>
          <span className="text-[var(--fg-subtle)]">→</span>
        </a>
      </div>
    </div>
  );
}
