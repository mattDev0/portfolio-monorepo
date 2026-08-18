export default function SpotifyPlayer({ spotifyData, progressPercent, localProgressMs, formatTime }) {
  const isPlaying = spotifyData?.is_playing;
  const isRecentlyPlayed = spotifyData?.is_recently_played;

  const innerContent = (
    <>
      <div className="absolute top-0 right-0 p-4 cursor-help group/tooltip">
        <span className="flex h-3 w-3 relative">
          <span className={`animate-ping absolute inline-flex h-full w-full rounded-full opacity-75 ${isPlaying ? 'bg-[var(--color-status-online)]' : isRecentlyPlayed ? 'bg-[var(--color-accent-amber)]' : 'bg-[var(--color-status-muted)]'}`}></span>
          {isPlaying && <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[var(--color-status-online)] opacity-40 delay-300"></span>}
          <span className={`relative inline-flex rounded-full h-3 w-3 ${isPlaying ? 'bg-[var(--color-status-online)]' : isRecentlyPlayed ? 'bg-[var(--color-status-warning)]' : 'bg-[var(--color-status-muted)]'} ${isPlaying ? 'shadow-[0_0_8px_#10b981]' : isRecentlyPlayed ? 'shadow-[0_0_8px_#f59e0b]' : ''}`}></span>
        </span>
        {/* Custom Tooltip */}
        <div className="absolute right-0 top-8 w-32 scale-0 group-hover/tooltip:scale-100 transition-all duration-200 origin-top-right rounded bg-[var(--color-bg-surface-elevated)] border border-[var(--color-accent-emerald-border)] p-2 text-center text-[10px] text-[var(--color-accent-emerald)] font-mono shadow-xl z-20">
          {isPlaying ? 'Live playback' : isRecentlyPlayed ? 'Playback idle' : 'Offline'}
        </div>
      </div>
      
      <h3 className="text-lg font-bold text-[var(--color-accent-emerald)] mb-0 tracking-wide flex items-center">
        <svg className="w-5 h-5 mr-2 text-[var(--color-accent-emerald)] group-hover:animate-spin" style={{ animationDuration: isPlaying ? '6s' : '0s' }} fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.4 0 0 5.4 0 12s5.4 12 12 12 12-5.4 12-12S18.66 0 12 0zm5.521 17.34c-.24.359-.66.48-1.021.24-2.82-1.74-6.36-2.101-10.561-1.141-.418.122-.779-.179-.899-.539-.12-.421.18-.78.54-.9 4.56-1.021 8.52-.6 11.64 1.32.42.18.54.659.3 1.02zm1.44-3.3c-.301.42-.841.6-1.262.3-3.239-1.98-8.159-2.58-11.939-1.38-.479.12-1.02-.12-1.14-.6-.12-.48.12-1.021.6-1.141C9.6 9.9 15 10.561 18.72 12.84c.361.181.54.84.241 1.2zM20.4 9.06C16.8 6.9 9.72 6.72 5.64 8.04c-.6.18-1.2-.12-1.38-.72-.18-.6.12-1.2.72-1.38 4.68-1.44 12.48-1.2 16.68 1.32.54.3.72.96.42 1.5-.24.54-.84.72-1.68.3z"/></svg>
        Spotify Session
      </h3>
      <p className="text-[10px] text-[var(--color-text-muted)] font-medium mb-5">Web API sync via Rust token auth</p>
      
      {spotifyData ? (
        <div>
          <div className="flex items-center space-x-4 mt-2">
            <div className={`w-16 h-16 bg-[var(--color-bg-tag)] rounded-lg flex-shrink-0 shadow-md overflow-hidden relative border border-[var(--color-border-primary)] transition-transform duration-500 ${isPlaying ? 'group-hover:rotate-12 group-hover:scale-105' : ''}`}>
              {spotifyData.album_art ? (
                <img src={spotifyData.album_art} alt="Album Art" className={`w-full h-full object-cover transition-all duration-500 ${isPlaying ? '' : 'grayscale opacity-60'}`} />
              ) : (
                <div className="absolute inset-0 bg-gradient-to-tr from-[var(--color-accent-emerald-solid)] to-[var(--color-bg-primary)] opacity-50"></div>
              )}
            </div>
            <div className="overflow-hidden flex-grow">
              <p className="text-[var(--color-text-primary)] font-bold truncate group-hover:text-[var(--color-accent-emerald)] transition-colors">{spotifyData.title}</p>
              <p className="text-[var(--color-text-tertiary)] text-sm truncate">{spotifyData.artist}</p>
              <p className={`text-[10px] mt-1 font-mono tracking-wider uppercase font-semibold ${isPlaying ? 'text-[var(--color-accent-emerald)]' : isRecentlyPlayed ? 'text-[var(--color-accent-amber)]' : 'text-[var(--color-text-muted)]'}`}>
                {isPlaying ? 'Now Playing' : isRecentlyPlayed ? 'Recently Played' : 'Offline'}
              </p>
            </div>
          </div>
          
          {isPlaying && spotifyData.duration_ms > 0 && (
            <div className="mt-5">
              <div className="w-full h-1 bg-[var(--color-bg-progress-track)] rounded-full overflow-hidden">
                <div 
                  className="h-full bg-[var(--color-accent-emerald-solid)] transition-all duration-1000 ease-linear shadow-[0_0_8px_var(--color-accent-emerald)]"
                  style={{ width: `${progressPercent}%` }}
                ></div>
              </div>
              <div className="flex justify-between text-[9px] text-[var(--color-text-muted)] mt-1.5 font-mono">
                <span>{formatTime(localProgressMs)}</span>
                <span>{formatTime(spotifyData.duration_ms)}</span>
              </div>
            </div>
          )}
        </div>
      ) : (
        <p className="text-[var(--color-text-muted)] animate-pulse font-mono text-xs mt-2">Connecting to Spotify Web API...</p>
      )}
    </>
  );

  if (spotifyData?.track_url) {
    return (
      <a 
        href={spotifyData.track_url}
        target="_blank"
        rel="noopener noreferrer"
        className="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-accent-emerald-border)] hover:border-[var(--color-accent-emerald)] relative overflow-hidden transition-all duration-300 group flex flex-col justify-center cursor-pointer"
        style={{ boxShadow: 'var(--shadow-card)' }}
      >
        {innerContent}
      </a>
    );
  }

  return (
    <section 
      className="hover-lift bg-[var(--color-bg-surface)] backdrop-blur-md p-6 rounded-2xl border border-[var(--color-border-primary)] hover:border-[var(--color-border-secondary)] relative overflow-hidden transition-all duration-300 flex flex-col justify-center"
      style={{ boxShadow: 'var(--shadow-card)' }}
    >
      {innerContent}
    </section>
  );
}
