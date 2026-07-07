import { Cpu, Cloud, Zap } from 'lucide-react';

export default function AboutSection({ config }) {
  const getIcon = (label) => {
    switch (label) {
      case 'Systems & OS':
        return <Cpu className="w-4 h-4 text-[var(--accent-primary)]" />;
      case 'Cloud & DevOps':
        return <Cloud className="w-4 h-4 text-[var(--accent-primary)]" />;
      case 'Backend':
        return <Zap className="w-4 h-4 text-[var(--accent-primary)]" />;
      default:
        return null;
    }
  };

  return (
    <section id="about" className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-6 md:p-8 flex flex-col justify-between">
      <div>
        <h2 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-semibold text-[var(--fg-default)] mb-4 pb-2 border-b border-[var(--border-muted)]">
          About the Developer
        </h2>
        
        <p className="text-[var(--text-base)] leading-[var(--lh-base)] text-[var(--fg-default)] whitespace-pre-line">
          {config.about}
        </p>

        {/* Highlights Horizontal List (Not Cards) */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-6 pt-6 border-t border-[var(--border-muted)]">
          {config.highlights && config.highlights.map((hl, i) => (
            <div key={i} className="flex items-start space-x-3">
              <div className="mt-0.5 shrink-0">
                {getIcon(hl.label)}
              </div>
              <div className="space-y-1">
                <h3 className="text-[var(--text-sm)] font-semibold text-[var(--fg-default)] leading-none">
                  {hl.label}
                </h3>
                <p className="text-[var(--text-xs)] text-[var(--fg-muted)] leading-[var(--lh-xs)]">
                  {hl.detail}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Education Compact Row */}
      {config.education && (
        <div className="mt-8 pt-6 border-t border-[var(--border-muted)] flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2">
          <div>
            <span className="text-[var(--text-xs)] font-bold text-[var(--accent-primary)] uppercase tracking-wider block mb-1">
              Education
            </span>
            <h3 className="text-[var(--text-sm)] font-semibold text-[var(--fg-default)]">
              {config.education.degree}
            </h3>
            <p className="text-[var(--text-xs)] text-[var(--fg-muted)]">
              {config.education.institution}
            </p>
          </div>
          <span className="text-[var(--text-sm)] font-semibold text-[var(--fg-muted)] sm:text-right">
            Class of {config.education.year}
          </span>
        </div>
      )}
    </section>
  );
}
