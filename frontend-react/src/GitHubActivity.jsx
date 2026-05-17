import React, { useState, useEffect } from 'react';

const GitHubActivity = () => {
  const [commits, setCommits] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Use Vite's native environment checker.
    // If running locally, hit the local Java server. If on Azure, use the relative path.
    const apiUrl = import.meta.env.DEV 
      ? 'http://localhost:8081/api/github/activity' 
      : '/api/github/activity'; 

    const fetchCommits = async () => {
      try {
        const response = await fetch(apiUrl);
        if (!response.ok) throw new Error('Failed to fetch activity');
        const data = await response.json();
        setCommits(data);
      } catch (err) {
        console.error("GitHub fetch error:", err);
        setError("Activity feed temporarily unavailable.");
      } finally {
        setLoading(false);
      }
    };

    fetchCommits();
  }, []);

  // Format the ISO date into a readable string (e.g., "May 17, 2026")
  const formatDate = (dateString) => {
    if (dateString === "Just now") return dateString;
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
  };

  return (
    <div className="bg-gray-800/50 border border-gray-700 rounded-xl p-6 mb-8 w-full">
      <div className="flex items-center mb-6">
        <svg className="w-6 h-6 text-blue-400 mr-3" fill="currentColor" viewBox="0 0 24 24">
          <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
        </svg>
        <h3 className="text-xl font-bold text-gray-200">Recent Github Commits(Java Infra)</h3>
      </div>

      {loading ? (
        <div className="animate-pulse space-y-4">
          {[1, 2, 3].map(i => (
            <div key={i} className="h-16 bg-gray-700/50 rounded-lg"></div>
          ))}
        </div>
      ) : error ? (
        <p className="text-gray-400 italic">{error}</p>
      ) : (
        <div className="space-y-4">
          {commits.map((commit, index) => (
            <div key={index} className="flex flex-col p-4 bg-gray-900/50 border border-gray-700/50 rounded-lg hover:border-gray-600 transition-colors">
              <div className="flex justify-between items-start mb-2">
                <span className="text-sm font-semibold text-blue-400">{commit.repo}</span>
                <span className="text-xs text-gray-500">{formatDate(commit.date)}</span>
              </div>
              <p className="text-sm text-gray-300 font-mono truncate">
                {commit.message}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default GitHubActivity;