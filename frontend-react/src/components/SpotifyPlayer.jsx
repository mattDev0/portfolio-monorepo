export default function SpotifyPlayer({ spotifyData, progressPercent, localProgressMs, formatTime }) {
  const isPlaying = spotifyData?.is_playing;
  const isRecentlyPlayed = spotifyData?.is_recently_played;

  const getStatusText = () => {
    if (isPlaying) return 'Now Playing';
    if (isRecentlyPlayed) return 'Recently Played';
    return 'Offline';
  };

  const getStatusColorClass = () => {
    if (isPlaying) return 'bg-[var(--status-success)]';
    if (isRecentlyPlayed) return 'bg-[var(--status-warning)]';
    return 'bg-[var(--fg-subtle)]';
  };

  const innerContent = (
    <div className="relative">
      {/* Top right status dot + text */}
      <div className="absolute top-0 right-0 flex items-center space-x-1.5">
        <span className={`w-2 h-2 rounded-full ${getStatusColorClass()}`} />
        <span className="text-[10px] text-[var(--fg-muted)] font-mono uppercase">
          {getStatusText()}
        </span>
      </div>
      
      <h3 className="text-sm font-semibold text-[var(--fg-default)] uppercase tracking-wider mb-1 flex items-center">
        <svg className="w-4 h-4 mr-2 text-[var(--fg-muted)]" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z"/>
        </svg>
        Spotify Session
      </h3>
      <p className="text-[10px] text-[var(--fg-subtle)] font-medium mb-5">Web API sync via Rust gateway</p>
      
      {spotifyData ? (
        <div>
          <div className="flex items-center space-x-4 mt-2">
            <div className="w-14 h-14 bg-[var(--bg-elevated)] rounded-[var(--radius-md)] flex-shrink-0 overflow-hidden relative border border-[var(--border-default)]">
              {spotifyData.album_art ? (
                <img src={spotifyData.album_art} alt="Album Art" className={`w-full h-full object-cover ${isPlaying ? '' : 'grayscale opacity-50'}`} />
              ) : (
                <div className="absolute inset-0 bg-gradient-to-tr from-[var(--status-success)]/20 to-[var(--bg-inset)] opacity-50"></div>
              )}
            </div>
            <div className="overflow-hidden flex-grow">
              <p className="text-[var(--fg-default)] font-semibold text-sm truncate">{spotifyData.title}</p>
              <p className="text-[var(--fg-muted)] text-xs truncate">{spotifyData.artist}</p>
            </div>
          </div>
          
          {isPlaying && spotifyData.duration_ms > 0 && (
            <div className="mt-4">
              <div className="w-full h-1 bg-[var(--bg-inset)] rounded-full overflow-hidden">
                <div 
                  className="h-full bg-[var(--status-success)] transition-all duration-1000 ease-linear"
                  style={{ width: `${progressPercent}%` }}
                ></div>
              </div>
              <div className="flex justify-between text-[9px] text-[var(--fg-subtle)] mt-1.5 font-mono">
                <span>{formatTime(localProgressMs)}</span>
                <span>{formatTime(spotifyData.duration_ms)}</span>
              </div>
            </div>
          )}
        </div>
      ) : (
        <div className="space-y-2 mt-4">
          <div className="h-4 w-3/4 skeleton rounded-[var(--radius-sm)]" />
          <div className="h-4 w-1/2 skeleton rounded-[var(--radius-sm)]" />
        </div>
      )}
    </div>
  );

  if (spotifyData?.track_url) {
    return (
      <a 
        href={spotifyData.track_url}
        target="_blank"
        rel="noopener noreferrer"
        className="bg-[var(--bg-surface)] border border-[var(--border-default)] hover:border-[var(--fg-subtle)] p-6 rounded-[var(--radius-lg)] block transition-colors duration-150 cursor-pointer focus-visible:outline-none"
      >
        {innerContent}
      </a>
    );
  }

  return (
    <div className="bg-[var(--bg-surface)] border border-[var(--border-default)] p-6 rounded-[var(--radius-lg)]">
      {innerContent}
    </div>
  );
}
