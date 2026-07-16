import React, { createContext, useContext, useState, useEffect } from 'react';

const ThemeContext = createContext(undefined);

export function ThemeProvider({ children }) {
  const [theme, setThemeState] = useState(() => {
    try {
      const stored = localStorage.getItem('theme');
      if (stored === 'light' || stored === 'dark' || stored === 'system') {
        return stored;
      }
    } catch (e) {
      console.warn('Failed to access localStorage for theme:', e);
    }
    return 'system';
  });

  const [resolvedTheme, setResolvedTheme] = useState('dark');

  useEffect(() => {
    const root = document.documentElement;
    
    const updateTheme = () => {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      const isDark = theme === 'dark' || (theme === 'system' && prefersDark);
      
      if (isDark) {
        root.classList.add('dark');
        setResolvedTheme('dark');
      } else {
        root.classList.remove('dark');
        setResolvedTheme('light');
      }
    };

    updateTheme();

    if (theme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      
      // Modern and fallback event listeners compatibility
      const handleChange = () => updateTheme();
      if (mediaQuery.addEventListener) {
        mediaQuery.addEventListener('change', handleChange);
      } else {
        mediaQuery.addListener(handleChange);
      }

      return () => {
        if (mediaQuery.removeEventListener) {
          mediaQuery.removeEventListener('change', handleChange);
        } else {
          mediaQuery.removeListener(handleChange);
        }
      };
    }
  }, [theme]);

  const setTheme = (newTheme) => {
    if (newTheme !== 'light' && newTheme !== 'dark' && newTheme !== 'system') return;
    setThemeState(newTheme);
    try {
      localStorage.setItem('theme', newTheme);
    } catch (e) {
      console.warn('Failed to write theme to localStorage:', e);
    }
  };

  return (
    <ThemeContext.Provider value={{ theme, resolvedTheme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}
