export default function SkillsSection({ skills }) {
  return (
    <section id="skills" className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-6 md:p-8">
      <h2 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-semibold text-[var(--fg-default)] mb-6 pb-2 border-b border-[var(--border-muted)]">
        Technical Skills
      </h2>
      
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-6">
        {skills && Object.entries(skills).map(([category, items]) => (
          <div key={category} className="space-y-3">
            <h3 className="text-[var(--text-xs)] font-bold text-[var(--accent-primary)] uppercase tracking-wider">
              {category}
            </h3>
            <div className="flex flex-wrap gap-2">
              {items.map((skill, index) => (
                <span
                  key={index}
                  className="bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] text-[var(--fg-default)] text-[var(--text-xs)] px-2.5 py-1 rounded-[var(--radius-sm)] transition-colors duration-150"
                >
                  {skill}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
