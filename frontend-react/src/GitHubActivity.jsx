import { useState, useEffect } from 'react';
import { AlertTriangle } from 'lucide-react';
import { apiUrl } from './api';

const GitHubActivity = () => {
  const [commits, setCommits] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [retryCounter, setRetryCounter] = useState(0);

  useEffect(() => {
    const fetchCommits = async () => {
      setLoading(true);
      setError(null);
      try {
        const activityUrl = apiUrl('java', '/api/github/activity');
        const response = await fetch(activityUrl);
        if (!response.ok) throw new Error('Failed to fetch activity');
        const data = await response.json();
        setCommits(data);
      } catch (err) {
        console.error("GitHub fetch error:", err);
        setError("Unable to connect. Retry.");
      } finally {
        setLoading(false);
      }
    };

    fetchCommits();
  }, [retryCounter]);

  return (
    <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] rounded-[var(--radius-lg)] p-6 w-full">
      <div className="flex items-center mb-6">
        <svg className="w-5 h-5 text-[var(--fg-muted)] mr-2.5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
        </svg>
        <h3 className="text-sm font-semibold text-[var(--fg-default)] uppercase tracking-wider">Recent GitHub Activity</h3>
      </div>

      {loading ? (
        <div className="space-y-4">
          {[1, 2, 3].map(i => (
            <div key={i} className="h-16 bg-[var(--bg-elevated)] border border-[var(--border-default)] rounded-[var(--radius-md)] skeleton" />
          ))}
        </div>
      ) : error ? (
        <div className="text-center py-6 flex flex-col items-center justify-center space-y-2">
          <AlertTriangle className="w-5 h-5 text-[var(--status-error)]" />
          <p className="text-[var(--text-sm)] text-[var(--fg-muted)]">{error}</p>
          <button
            onClick={() => setRetryCounter(prev => prev + 1)}
            className="px-3 py-1 bg-[var(--bg-elevated)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] text-xs text-[var(--fg-default)] rounded-[var(--radius-md)] cursor-pointer"
          >
            Retry
          </button>
        </div>
      ) : commits.length === 0 ? (
        <div className="text-center py-8 text-[var(--fg-subtle)] font-mono text-xs">
          No recent activity.
        </div>
      ) : (
        <ul className="space-y-4">
          {commits.map((commit, index) => (
            <li key={index} className="pb-4 border-b border-[var(--border-muted)] last:border-0 last:pb-0">
              {/* Repo Name & Date */}
              <div className="flex justify-between items-center mb-1.5">
                <span className="text-[var(--accent-primary)] font-semibold text-xs truncate pr-4">
                  {commit.repo}
                </span>
                <span className="text-[var(--fg-subtle)] text-[10px] font-medium font-mono">
                  {new Date(commit.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}
                </span>
              </div>

              {/* Hash Badge & Commit Message */}
              <div className="flex items-center space-x-2">
                <span className="bg-[var(--bg-elevated)] border border-[var(--border-default)] text-[var(--fg-muted)] font-mono text-[9px] px-1.5 py-0.5 rounded">
                  {commit.hash}
                </span>
                <span className="text-[var(--fg-default)] text-xs truncate">
                  {commit.message}
                </span>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

export default GitHubActivity;
