export default function ExperienceSection({ experience }) {
  return (
    <section id="experience" className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-6 md:p-8">
      <h2 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-semibold text-[var(--fg-default)] mb-6 pb-2 border-b border-[var(--border-muted)]">
        Experience
      </h2>
      
      <ol className="relative border-l-2 border-[var(--border-default)] ml-2.5 space-y-6">
        {experience && experience.map((exp, index) => (
          <li key={index} className="relative pl-6">
            {/* Timeline Dot */}
            <div className="absolute w-2.5 h-2.5 bg-[var(--accent-primary)] rounded-full -left-[6px] top-1.5" />
            
            <div className="flex flex-col sm:flex-row sm:items-baseline sm:justify-between gap-1 mb-2">
              <h3 className="text-[var(--text-md)] font-semibold text-[var(--fg-default)]">
                {exp.role}
              </h3>
              <span className="text-[var(--text-sm)] font-medium text-[var(--fg-muted)]">
                {exp.company}
              </span>
            </div>
            
            <p className="text-[var(--text-sm)] leading-relaxed text-[var(--fg-muted)]">
              {exp.description}
            </p>
          </li>
        ))}
      </ol>
    </section>
  );
}
