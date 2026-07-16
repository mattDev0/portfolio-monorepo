import { useTheme } from '../context/ThemeContext';

export default function ThemeToggle() {
  const { theme, resolvedTheme, setTheme } = useTheme();

  const cycleTheme = () => {
    const cycleMap = {
      system: 'light',
      light: 'dark',
      dark: 'system',
    };
    setTheme(cycleMap[theme]);
  };

  // Dynamic accessibility label
  const ariaLabel = `Theme: ${theme}. Click to switch to ${
    theme === 'system' ? 'light' : theme === 'light' ? 'dark' : 'system'
  } mode.`;

  return (
    <button
      onClick={cycleTheme}
      className="relative w-9 h-9 rounded-lg border border-[var(--color-border-primary)] bg-[var(--color-hover-bg)] text-[var(--color-text-tertiary)] hover:text-[var(--color-accent-emerald)] hover:border-[var(--color-border-secondary)] transition-all duration-200 cursor-pointer flex items-center justify-center focus-visible:outline-2 focus-visible:outline-[var(--color-focus-ring)] focus-visible:outline-offset-2"
      aria-label={ariaLabel}
      title={`Active Theme: ${theme.toUpperCase()} (Resolved: ${resolvedTheme.toUpperCase()})`}
    >
      <div className="relative w-5 h-5 flex items-center justify-center overflow-hidden">
        {/* Sun Icon */}
        <span
          className={`absolute inset-0 flex items-center justify-center transition-all duration-300 ${
            theme === 'light'
              ? 'opacity-100 transform rotate-0 scale-100'
              : 'opacity-0 transform -rotate-90 scale-50 pointer-events-none'
          }`}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className="w-5 h-5"
          >
            <circle cx="12" cy="12" r="4"></circle>
            <path d="M12 2v2"></path>
            <path d="M12 20v2"></path>
            <path d="M4.93 4.93l1.41 1.41"></path>
            <path d="M17.66 17.66l1.41 1.41"></path>
            <path d="M2 12h2"></path>
            <path d="M20 12h2"></path>
            <path d="M6.34 17.66l-1.41 1.41"></path>
            <path d="M19.07 4.93l-1.41 1.41"></path>
          </svg>
        </span>

        {/* Moon Icon */}
        <span
          className={`absolute inset-0 flex items-center justify-center transition-all duration-300 ${
            theme === 'dark'
              ? 'opacity-100 transform rotate-0 scale-100'
              : 'opacity-0 transform -rotate-90 scale-50 pointer-events-none'
          }`}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className="w-5 h-5"
          >
            <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
          </svg>
        </span>

        {/* System Monitor Icon */}
        <span
          className={`absolute inset-0 flex items-center justify-center transition-all duration-300 ${
            theme === 'system'
              ? 'opacity-100 transform rotate-0 scale-100'
              : 'opacity-0 transform -rotate-90 scale-50 pointer-events-none'
          }`}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
            className="w-5 h-5"
          >
            <rect width="20" height="14" x="2" y="3" rx="2"></rect>
            <path d="M8 21h8"></path>
            <path d="M12 17v4"></path>
          </svg>
        </span>
      </div>
    </button>
  );
}
