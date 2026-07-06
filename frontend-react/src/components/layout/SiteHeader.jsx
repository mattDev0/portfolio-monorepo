import { useState, useEffect, useRef } from 'react';
import { Menu, X } from 'lucide-react';
import ThemeToggle from '../ui/ThemeToggle';

const SECTIONS = [
  { id: 'about', label: 'About' },
  { id: 'projects', label: 'Projects' },
  { id: 'skills', label: 'Skills' },
  { id: 'experience', label: 'Experience' },
  { id: 'infrastructure', label: 'Infrastructure' }
];

export default function SiteHeader({ activeSection, siteName }) {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const mobileMenuRef = useRef(null);

  // Close mobile menu on escape key
  useEffect(() => {
    const handleKeyDown = (e) => {
      if (e.key === 'Escape') {
        setMobileMenuOpen(false);
      }
    };
    if (mobileMenuOpen) {
      window.addEventListener('keydown', handleKeyDown);
    }
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [mobileMenuOpen]);

  // Prevent background scroll when mobile menu is open
  useEffect(() => {
    if (mobileMenuOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => {
      document.body.style.overflow = '';
    };
  }, [mobileMenuOpen]);

  return (
    <>
      {/* Skip to Main Content Link */}
      <a
        href="#main-content"
        className="sr-only focus:not-sr-only focus:fixed focus:top-4 focus:left-4 focus:z-50 focus:px-4 focus:py-2 focus:bg-[var(--accent-primary)] focus:text-white focus:rounded-[var(--radius-md)] focus:outline-none"
      >
        Skip to content
      </a>

      <header className="fixed top-0 left-0 right-0 z-45 h-14 bg-[var(--bg-surface)] border-b border-[var(--border-default)]">
        <div className="max-w-5xl mx-auto h-full px-4 sm:px-6 lg:px-8 flex items-center justify-between">
          
          {/* Logo / Site Name */}
          <a href="#" className="font-semibold text-sm text-[var(--fg-default)] hover:text-[var(--accent-primary)] transition-colors focus-visible:outline-none">
            {siteName}
          </a>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex items-center space-x-6 h-full">
            {SECTIONS.map((sec) => {
              const isActive = activeSection === sec.id;
              return (
                <a
                  key={sec.id}
                  href={`#${sec.id}`}
                  aria-current={isActive ? 'page' : undefined}
                  className={`text-sm relative flex items-center h-full border-b-2 px-1 transition-colors focus-visible:outline-none ${
                    isActive 
                      ? 'text-[var(--accent-primary)] border-[var(--accent-primary)] font-medium' 
                      : 'text-[var(--fg-muted)] border-transparent hover:text-[var(--fg-default)]'
                  }`}
                >
                  {sec.label}
                </a>
              );
            })}
          </nav>

          {/* Right Side: Theme Toggle & Mobile Menu Trigger */}
          <div className="flex items-center space-x-2">
            <ThemeToggle />
            
            {/* Mobile Hamburger Button */}
            <button
              onClick={() => setMobileMenuOpen(true)}
              aria-label="Open navigation menu"
              aria-expanded={mobileMenuOpen}
              className="md:hidden w-8 h-8 flex items-center justify-center rounded-[var(--radius-md)] text-[var(--fg-muted)] hover:text-[var(--fg-default)] hover:bg-[var(--bg-elevated)] cursor-pointer focus-visible:outline-none"
            >
              <Menu className="w-5 h-5" />
            </button>
          </div>

        </div>
      </header>

      {/* Mobile Menu Overlay */}
      {mobileMenuOpen && (
        <div
          ref={mobileMenuRef}
          className="fixed inset-0 z-50 bg-[var(--bg-surface)] flex flex-col md:hidden"
          role="dialog"
          aria-modal="true"
          aria-label="Navigation menu"
        >
          {/* Mobile Header Bar inside Modal */}
          <div className="h-14 px-4 flex items-center justify-between border-b border-[var(--border-default)]">
            <span className="font-semibold text-sm text-[var(--fg-default)]">
              Navigation
            </span>
            <button
              onClick={() => setMobileMenuOpen(false)}
              aria-label="Close navigation menu"
              className="w-8 h-8 flex items-center justify-center rounded-[var(--radius-md)] text-[var(--fg-muted)] hover:text-[var(--fg-default)] hover:bg-[var(--bg-elevated)] cursor-pointer focus-visible:outline-none"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          {/* Mobile Navigation List */}
          <nav className="flex-1 flex flex-col p-6 space-y-4 overflow-y-auto">
            {SECTIONS.map((sec) => {
              const isActive = activeSection === sec.id;
              return (
                <a
                  key={sec.id}
                  href={`#${sec.id}`}
                  onClick={() => setMobileMenuOpen(false)}
                  aria-current={isActive ? 'page' : undefined}
                  className={`text-base py-3 px-4 rounded-[var(--radius-md)] flex items-center min-h-[44px] transition-colors focus-visible:outline-none ${
                    isActive
                      ? 'bg-[var(--bg-elevated)] text-[var(--accent-primary)] font-semibold'
                      : 'text-[var(--fg-muted)] hover:text-[var(--fg-default)] hover:bg-[var(--bg-elevated)]/50'
                  }`}
                >
                  {sec.label}
                </a>
              );
            })}
          </nav>
        </div>
      )}
    </>
  );
}
