import { MapPin, Mail } from 'lucide-react';

export default function HeroSection({ config }) {
  const avatarUrl = 'https://github.com/mattDev0.png';

  return (
    <section className="w-full max-w-5xl mx-auto flex flex-col-reverse md:flex-row md:items-center md:justify-between gap-8 md:gap-12 mb-[var(--space-16)]">
      
      {/* Left Column: Text & Actions */}
      <div className="flex-1 text-center md:text-left space-y-4 md:space-y-6">
        
        <div className="space-y-2">
          <h1 className="text-[var(--text-display)] leading-[var(--lh-display)] font-bold text-[var(--fg-default)] tracking-tight">
            {config.name}
          </h1>
          <h2 className="text-[var(--text-lg)] leading-[var(--lh-lg)] font-normal text-[var(--fg-muted)]">
            {config.title}
          </h2>
        </div>

        {/* Location and Remote Status */}
        <div className="flex flex-wrap items-center justify-center md:justify-start gap-3 text-sm">
          <div className="flex items-center text-[var(--fg-muted)]">
            <MapPin className="w-4 h-4 mr-1.5 text-[var(--accent-primary)]" />
            <span className="font-medium text-[var(--fg-default)]">{config.location}</span>
          </div>

          {config.openToRemote && (
            <span className="inline-flex items-center px-2 py-0.5 rounded-[var(--radius-sm)] text-xs font-semibold uppercase tracking-wider bg-[var(--status-success)]/10 text-[var(--status-success)] border border-[var(--status-success)]/20">
              Open to Remote
            </span>
          )}
        </div>

        {/* Tagline */}
        <p className="text-[var(--text-base)] leading-[var(--lh-base)] text-[var(--fg-muted)] max-w-xl">
          {config.tagline}
        </p>

        {/* Action Buttons */}
        <div className="flex flex-col sm:flex-row items-center gap-3 justify-center md:justify-start w-full sm:w-auto">
          <a
            href={`mailto:${config.email}`}
            className="inline-flex items-center justify-center space-x-2 px-5 py-2.5 bg-[var(--accent-primary)] hover:bg-[var(--accent-primary-hover)] text-white font-semibold text-sm rounded-[var(--radius-md)] transition-colors focus-visible:outline-none w-full sm:w-auto cursor-pointer"
          >
            <Mail className="w-4 h-4" />
            <span>Email Me</span>
          </a>
          <a
            href={config.githubUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center space-x-2 px-5 py-2.5 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] text-[var(--fg-default)] hover:bg-[var(--bg-surface)] font-semibold text-sm rounded-[var(--radius-md)] transition-colors focus-visible:outline-none w-full sm:w-auto cursor-pointer"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path fillRule="evenodd" clipRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
            </svg>
            <span>GitHub</span>
          </a>
        </div>

      </div>

      {/* Right Column: Portrait Image */}
      <div className="flex justify-center md:justify-end">
        <img
          src={avatarUrl}
          alt={config.name}
          className="w-24 h-24 rounded-full border border-[var(--border-default)] p-1 bg-[var(--bg-surface)] object-cover"
        />
      </div>

    </section>
  );
}
